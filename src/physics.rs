use crate::particle::Particle;


use dlt::tensor::*;
use dlt::dimension::*;
use dlt::units::*;
use dlt::si::*;
use dlt::unit_div;




pub struct CollisionInfo {
    pub ke: Scalar<f32,Energy>,
    pub normal: Vec2<f32,Length>,
}

pub fn apply_solid_collision(particle: &mut Particle, info: CollisionInfo) {

}   