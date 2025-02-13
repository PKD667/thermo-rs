use crate::linear::v2d;
use crate::measure;
use crate::particle::Particle;


use crate::segment::Segment;

use uom::si::f64::ThermodynamicTemperature;
use uom::si::thermodynamic_temperature;



pub struct Shape {
    pub segments: Vec<Segment>,

    pub temperature: ThermodynamicTemperature,
    pub conductivity: f64,
}

impl Shape {
    pub fn new(points: Vec<v2d>) -> Shape {

        let segments: Vec<Segment> = vec![];
        // transform points into segments
        // find an algorithm

        Shape {
            segments,
            temperature: ThermodynamicTemperature::new::<thermodynamic_temperature::kelvin>(0.0),
            conductivity: 1.0,
        }
    }


    pub fn collide(&mut self, particle: &mut Particle) {
        for segment in self.segments.iter() {
            if let Some(collision_info) = segment.collision(particle) {
                // compute the energy transfer
            }
        }
        
    }




    pub fn heat(&mut self, energy: f64) {
        self.temperature_energy += energy;
    }
}


