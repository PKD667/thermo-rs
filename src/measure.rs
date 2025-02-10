use crate::particle;
use crate::system;


pub struct Measurer {
    time: f64,

    wall_hits_energy: Vec<(f64,f64)>,
}



impl Measurer {
    pub fn new() -> Measurer {
        Measurer {
            time: 0.0,

            wall_hits_energy: Vec::new(),
        }
    }

    pub fn record_wall_hit(&mut self, particle: &particle::Particle) {
        self.wall_hits_energy.push((particle.mass as f64 * particle.vel.norm() * particle.vel.norm(),self.get_time()));
    }

    pub fn record_time(&mut self, dt: f64) {
        self.time += dt;
    }

    pub fn get_pressure(&self, system: &system::System) -> f64 {

        let sample = 10.0;

        // get all hits in the sample
        let mut hits = Vec::new();
        let mut oldest_hit = self.get_time();
        for i in 0..self.wall_hits_energy.len() {
            if self.get_time() - self.wall_hits_energy[i].1 < sample  {
                hits.push(self.wall_hits_energy[i].0);
                if self.wall_hits_energy[i].1 < oldest_hit {
                    oldest_hit = self.wall_hits_energy[i].1;
                }
            } 
        }

        // get average
        let sum = hits.iter().sum::<f64>();

        // get surface
        let surface = (system.width + system.height) * 2.0;

        println!("delta time: {}", self.get_time() - oldest_hit as f64);

        // get pressure
        (sum / surface) / (self.get_time() - oldest_hit) as f64

    }

    pub fn get_kinetic_energy(&self, system: &system::System) -> f64 {
        system.particles.iter().map(|p| 0.5 * p.mass as f64 * p.vel.norm() * p.vel.norm()).sum()
    }

    pub fn get_temperature(&self, system: &system::System) -> f64 {
        let n = system.particles.len() as f64;
        let ke = self.get_kinetic_energy(system);
        2.0 * ke / (3.0 * n)
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }
}