
use crate::particle::Particle;


use crate::segment::Segment;

use dlt::tensor::*;
use dlt::dimension::*;
use dlt::units::*;
use dlt::si::*;

pub struct Shape {
    pub segments: Vec<Segment>,


    thermal_capacity: Scalar<HeatCapacity>,
    temperature: Scalar<Temperature>,
}

impl Shape {
    pub fn new(points: Vec<Vec2<Length>>,capacity: Scalar::<HeatCapacity>) -> Shape {

        let segments: Vec<Segment> = vec![];
        // transform points into segments
        // find an algorithm

        Shape {
            segments,
            temperature: Scalar::<Temperature>::zero(),
            thermal_capacity: capacity, 
        }
    }


    pub fn collide(&mut self, particle: &mut Particle) {
        for segment in self.segments.iter() {
            if let Some(collision_info) = segment.collision(particle) {
                // compute the energy transfer
            }  
        }
        
    }

    pub fn heat(&mut self, energy: Scalar<Energy>) {

        self.temperature += energy * self.thermal_capacity.inv();
    }
}


