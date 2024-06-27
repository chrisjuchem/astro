mod ui;

use crate::ui::{add_ui, update_ui};
use bevy::input::common_conditions::input_pressed;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rand_distr::StandardNormal;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    // app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app.add_systems(Startup, setup);
    app.add_systems(Startup, add_ui);

    app.add_systems(
        Update,
        (
            camera_controller.run_if(input_pressed(MouseButton::Right)),
            update_ui,
        )
            .chain(),
    );

    app.init_resource::<Angles>();
    app.insert_resource(ClearColor(Color::BLACK));

    app.run();
}

#[derive(Resource, Deref, DerefMut, Default)]
struct Angles(Vec2);

#[derive(Component)]
struct Planet;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let star_mesh = meshes.add(Sphere::new(0.01).mesh().build());
    let star_mat = materials.add(StandardMaterial {
        // emissive: LinearRgba::rgb(20., 20., 20.),
        emissive: Color::rgb(20., 20., 20.),
        ..default()
    });

    let mut rng = SmallRng::seed_from_u64(0xab54_397f);

    for _ in 0..4000 {
        let pos = Vec3::new(
            rng.sample(StandardNormal),
            rng.sample(StandardNormal),
            rng.sample(StandardNormal),
        ) * 1_000_000_000.; // AU

        commands.spawn(MaterialMeshBundle {
            mesh: star_mesh.clone(),
            material: star_mat.clone(),
            transform: Transform::from_translation(pos).with_scale(Vec3::splat(100000000.)),
            ..default()
        });
    }

    commands
        .spawn((Name::new("Planet"), Planet, SpatialBundle::default()))
        .with_children(|cmd| {
            let size = 0.0010;

            let planet_mesh = meshes.add(Sphere::new(1.0).mesh().ico(32).unwrap());
            cmd.spawn((MaterialMeshBundle {
                mesh: planet_mesh,
                material: star_mat.clone(),
                transform: Transform::from_scale(Vec3::splat(size)),
                ..default()
            },));
            cmd.spawn((
                Camera3dBundle {
                    projection: Projection::Perspective(PerspectiveProjection {
                        fov: std::f32::consts::PI / 4.,
                        aspect_ratio: 1.,
                        near: 0.0,
                        far: 10000000000.0,
                    }),
                    transform: Transform::from_xyz(0., size + 0.00000001, 0.),
                    ..default()
                },
                Name::new("Camera"),
            ));
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
