mod sim;
mod ui;
mod util;

use crate::sim::{simulation, toggle_sim, Now, Orbit, SimState, SimTime};
use crate::ui::{add_ui, update_ui};
use bevy::input::common_conditions::input_pressed;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rand_distr::StandardNormal;

const AU: f32 = 10000.;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app.add_systems(Startup, setup);
    app.add_systems(Startup, add_ui);

    app.add_systems(
        Update,
        (
            camera_controller.run_if(input_pressed(MouseButton::Right)),
            toggle_sim,
            update_ui,
        )
            .chain(),
    );
    app.add_systems(FixedUpdate, simulation);

    app.register_type::<Now>();
    app.init_resource::<Now>();
    app.init_resource::<SimState>();
    app.init_resource::<Angles>();
    app.insert_resource(ClearColor(Color::BLACK));

    app.run();
}

#[derive(Resource, Deref, DerefMut, Default)]
struct Angles(Vec2);

#[derive(Component)]
struct Planet {
    axis: Vec3,
}

fn random_dir(rng: &mut SmallRng) -> Vec3 {
    Vec3::new(
        rng.sample(StandardNormal),
        rng.sample(StandardNormal),
        rng.sample(StandardNormal),
    )
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let star_mesh = meshes.add(Sphere::new(0.01 * AU).mesh().build());
    let star_mat = materials.add(StandardMaterial {
        // emissive: LinearRgba::rgb(20., 20., 20.),
        emissive: Color::rgb(20., 20., 20.),
        ..default()
    });

    let mut rng = SmallRng::seed_from_u64(0xab54_397f);

    for _ in 0..4000 {
        let pos = random_dir(&mut rng) * 1_000_000_000. * AU; // AU

        commands.spawn(MaterialMeshBundle {
            mesh: star_mesh.clone(),
            material: star_mat.clone(),
            transform: Transform::from_translation(pos * AU)
                .with_scale(/*fudge brightness*/ Vec3::splat(100_000_000. * AU)),
            ..default()
        });
    }

    commands.spawn((
        Name::new("Sun"),
        MaterialMeshBundle {
            mesh: star_mesh.clone(),
            material: star_mat.clone(),
            transform: Transform::from_scale(Vec3::splat(10.)),
            ..default()
        },
    ));

    commands
        .spawn((
            Name::new("Comet orbit"),
            SpatialBundle {
                transform: Transform::default().looking_to(random_dir(&mut rng), Vec3::Y),
                ..default()
            },
        ))
        .with_children(|cmd| {
            cmd.spawn((
                Name::new("Comet"),
                MaterialMeshBundle {
                    mesh: star_mesh.clone(),
                    material: star_mat.clone(),
                    transform: Transform::from_scale(Vec3::splat(3.)),
                    ..default()
                },
                Orbit {
                    ellipse: Ellipse::new(AU, 4. * AU / 3.),
                    period: SimTime::from_secs(20),
                    starting_offset: -1.,
                },
            ));
        });

    commands
        .spawn((
            Name::new("Planet"),
            Planet {
                axis: Vec3::new(0.4, 0.9, 0.).normalize(),
            },
            SpatialBundle {
                transform: Transform::from_translation(Vec3::Z * AU),
                ..default()
            },
        ))
        .with_children(|cmd| {
            let size = 1e-3 * AU; // should be -5 but it flickers :(

            let planet_mesh = meshes.add(Sphere::new(1.0).mesh().ico(32).unwrap());
            cmd.spawn((MaterialMeshBundle {
                mesh: planet_mesh,
                material: star_mat.clone(),
                transform: Transform::from_scale(Vec3::splat(size)),
                ..default()
            },));

            cmd.spawn((
                SpatialBundle {
                    transform: {
                        let mut t = Transform::from_xyz(0., size * 1.0001, 0.);
                        t.rotate_around(
                            Vec3::ZERO,
                            Quat::from_axis_angle(random_dir(&mut rng).normalize(), 2.),
                        );
                        t
                    },
                    ..default()
                },
                Name::new("Camera Anchor"),
            ))
            .with_children(|cmd| {
                cmd.spawn((
                    Camera3dBundle {
                        projection: Projection::Perspective(PerspectiveProjection {
                            fov: std::f32::consts::PI / 4.,
                            aspect_ratio: 1.,
                            near: 0.0,
                            far: 10000000000.0 * AU,
                        }),
                        ..default()
                    },
                    Name::new("Telescope Camera"),
                ));
            });
        });
}

fn camera_controller(
    mut cams: Query<&mut Transform, With<Camera>>,
    mut motion: EventReader<MouseMotion>,
    mut angles: ResMut<Angles>,
) {
    for e in motion.read() {
        angles.x += e.delta.x * 0.001;
        angles.x = angles.x.rem_euclid(std::f32::consts::PI * 2.);

        angles.y = (angles.y + (e.delta.y * 0.001))
            .min(std::f32::consts::PI / 2.)
            .max(-std::f32::consts::PI / 16.);
    }
    cams.single_mut().rotation = Quat::from_euler(EulerRot::YXZ, angles.x, angles.y, 0.);
}
