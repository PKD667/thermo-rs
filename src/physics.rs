use crate::particle::Particle;


use dlt::tensor::*;
use dlt::dimension::*;
use dlt::units::*;
use dlt::*;

// define the dimension of the Boltzmann constant

pub fn k_b() -> Scalar<Entropy> {
    Scalar::<Entropy>::new::<unit_div!(Joule,Kelvin)>([1.380649e-23])
}



pub struct CollisionInfo {
    pub ke: Scalar<Energy>,
    pub normal: Vec2<Length>,
}

pub fn apply_solid_collision(particle: &mut Particle, info: CollisionInfo) {

}   