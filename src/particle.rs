
use dlt::tensor::*;
use dlt::dimension::*;
use dlt::units::*;

pub struct Particle {
    pub pos: Vec2<Length>,
    pub vel: Vec2<Velocity>,
    pub mass: Scalar<Mass>,
    pub radius: Scalar<Length>,
}

impl Particle {
    pub fn new(pos: Vec2<Length>, vel: Vec2<Velocity>, mass: Scalar<Mass>, radius: Scalar<Length>) -> Particle {
        Particle {
            pos,
            vel,
            mass,
            radius,
        }
    }

    // utility wrappers
    pub fn dist(&self, other: &Particle) -> Scalar<Length> {
        self.pos.dist(other.pos)
    }

    pub fn update(&mut self, dt: Scalar<Time>) {
        self.pos = self.pos + self.vel.scale(dt);
    }

    pub fn apply(&mut self, force: Vec2<Force>, dt: Scalar<Time>) {
        
        let inv_mass = self.mass.inv();
        let acc = force.scale(inv_mass);

        self.vel = self.vel + acc.scale(dt);
    }

}
