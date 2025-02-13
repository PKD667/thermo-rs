use crate::particle::Particle;
use crate::linear::v2d;

use uom::si::f64::Energy;
use uom::si::energy;

pub const K_B: f64 = 1.380649e-23;

pub struct CollisionInfo {
    pub ke: Energy,
    pub normal: v2d,
}

pub fn apply_solid_collision(particle: &mut Particle, info: CollisionInfo) {

    let v_thermal = (2 * K_B *)
}   