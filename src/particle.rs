use crate::math::v2d;

pub struct Particle {
    pub pos: v2d,
    pub vel: v2d,
    pub mass: f64,
    pub radius: f64,
}

impl Particle {
    pub fn new(pos: v2d, vel: v2d, mass: f64, radius: f64) -> Particle {
        Particle {
            pos,
            vel,
            mass,
            radius,
        }
    }

    // utility wrappers
    pub fn dist(&self, other: &Particle) -> f64 {
        self.pos.dist(&other.pos)
    }

    pub fn update(&mut self, dt: f64) {
        self.pos = self.pos.add(&self.vel.mul(dt));
    }

    pub fn apply(&mut self, force: &v2d) {
        let acc = v2d::new(force.x / self.mass, force.y / self.mass);
        self.vel = self.vel.add(&acc);
    }

    // reserved for future use
}
