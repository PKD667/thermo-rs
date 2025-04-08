
use dlt::tensor::*;
use dlt::dimension::*;
use dlt::si::*;

pub struct Particle {
    pub pos: Vec2<f32,Length>,
    pub vel: Vec2<f32,Velocity>,
    pub acc: Vec2<f32,Acceleration>,
    pub mass: Scalar<f32,Mass>,
    pub radius: Scalar<f32,Length>,
}

impl Particle {
    pub fn new(pos: Vec2<f32,Length>, vel: Vec2<f32,Velocity>, mass: Scalar<f32,Mass>, radius: Scalar<f32,Length>) -> Particle {
        Particle {
            pos,
            vel,
            acc: Vec2::<f32,Acceleration>::zero(),
            mass,
            radius,
        }
    }

    // utility wrappers
    pub fn dist(&self, other: &Particle) -> Scalar<f32,Length> {
        self.pos.dist(other.pos).cast::<f32>()
    }

    pub fn update(&mut self, dt: Scalar<f32,Time>) {
        self.pos = self.pos + self.vel.scale(dt);
        self.vel = self.vel + self.acc.scale(dt);
        self.acc = Vec2::<f32,Acceleration>::zero();
    }

    pub fn apply(&mut self, force: Vec2<f32,Force>, dt: Scalar<f32,Time>) {
        
        self.acc = force / self.mass;
    }

}
