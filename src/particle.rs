use crate::linear::{
    LengthV2D,
    ForceV2D,
    VelocityV2D
}
use uom::si::f64::{Length, Mass, Velocity,Time};

// Use the concrete dimension type for Length and Velocity.
pub struct Particle {
    pub pos: LengthV2D,
    pub vel: VelocityV2D,
    pub mass: Mass,
    pub radius: Length,
}


impl Particle {
    pub fn new(pos: LengthV2D, vel: VelocityV2D, mass: Mass, radius: Length) -> Particle {
        Particle {
            pos,
            vel,
            mass,
            radius,
        }
    }

    // utility wrappers
    pub fn dist(&self, other: &Particle) -> Length {
        Length::new::<uom::si::length::meter>(self.pos.dist(&other.pos))
    }

    pub fn update(&mut self, dt: Time) {
        self.pos = self.pos + self.vel.scalar_mul(dt);
    }

    pub fn apply(&mut self, force: ForceV2D) {
        let acc = force / self.mass;
        self.vel = self.vel + &acc;
    }

}
