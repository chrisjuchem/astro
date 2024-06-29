use crate::util::bessel_j;
use crate::Planet;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SimState {
    rotation: bool,
    revolution: bool,
    time: bool,
}

pub fn toggle_sim(k: Res<ButtonInput<KeyCode>>, mut sim: ResMut<SimState>) {
    if k.just_pressed(KeyCode::KeyR) {
        sim.rotation = !sim.rotation;
    }
    if k.just_pressed(KeyCode::KeyE) {
        sim.revolution = !sim.revolution;
    }
    if k.just_pressed(KeyCode::KeyT) {
        sim.time = !sim.time;
    }
}

pub fn simulation(
    mut planets: Query<(&mut Transform, &Planet), Without<Orbit>>,
    sim: Res<SimState>,
    mut orbits: Query<(&mut Transform, &Orbit)>,
    mut now: ResMut<Now>,
) {
    if sim.time {
        now.tick();
    }

    for (mut t, p) in &mut planets {
        if sim.rotation {
            t.rotate_axis(p.axis, 0.03);
        }
        if sim.revolution {
            t.translate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, 0.02));
        }
    }

    for (mut t, o) in &mut orbits {
        t.translation = o.position(**now).extend(0.);
        t.translation.x -= o.c();
    }
}

#[derive(Component)]
pub struct Orbit {
    pub ellipse: Ellipse,
    pub period: SimTime,
    pub starting_offset: f32, //radians
}

impl Orbit {
    #[inline]
    pub fn a(&self) -> f32 {
        self.ellipse.semi_major()
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.ellipse.semi_minor()
    }

    /// Foci distance
    pub fn c(&self) -> f32 {
        let a_sq = self.a() * self.a();
        let b_sq = self.b() * self.b();
        (a_sq - b_sq).sqrt()
    }

    /// Eccentricity
    pub fn e(&self) -> f32 {
        self.c() / self.a()
    }

    /// Position relative to ellipse center given an angle (eccentric anomaly)
    pub fn angular_position(&self, angle_radians: f32) -> Vec2 {
        Vec2::new(
            self.a() * f32::cos(angle_radians),
            self.b() * f32::sin(angle_radians),
        )
    }

    pub fn position(&self, time: SimTime) -> Vec2 {
        let mean_anomaly = (time % self.period) / self.period * 2. * std::f32::consts::PI;
        let mean_anomaly = mean_anomaly + self.starting_offset;

        // https://en.wikipedia.org/wiki/Eccentric_anomaly#From_the_mean_anomaly
        let mut correction = 0.;
        for n in 1..10 {
            let nf = n as f32;
            correction += bessel_j(n, nf * self.e()) * f32::sin(nf * mean_anomaly) / nf;
        }
        let eccentric_anomaly = mean_anomaly + 2. * correction;
        self.angular_position(eccentric_anomaly)
    }
}

#[derive(Reflect, Resource, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub struct Now(pub SimTime);

#[derive(Copy, Clone, Eq, PartialEq, Default, Reflect)]
pub struct SimTime(u64);
impl SimTime {
    const FRAC_BITS: usize = 6;
    const FACTOR: u64 = 64;

    pub fn from_secs(secs: u64) -> SimTime {
        if secs != (secs << Self::FRAC_BITS) >> Self::FRAC_BITS {
            warn!("Constructed invalid SimTime: {secs}")
        }
        SimTime(secs << Self::FRAC_BITS)
    }

    pub fn tick(&mut self) {
        self.0 += 1;
    }
}

impl std::ops::Rem for SimTime {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl std::ops::Div for SimTime {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 as f32 / rhs.0 as f32
    }
}
