// compute and analyse particle - segments collisions

use crate::linear::*;
use crate::particle::Particle;
use crate::physics::CollisionInfo;

pub struct Segment {
    pub p1: v2d,
    pub p2: v2d,

    pub normal: v2d,
}


impl Segment {

    pub fn collision(&self, particle: &Particle) -> Option<CollisionInfo> {
        // 1. Find the closest point on the segment to our circle's center
        let segment_vec = self.p2.sub(&self.p1);
        let point_vec = particle.pos.sub(&self.p1);
    
        // Project point onto segment
        let t = point_vec.dot(&segment_vec) / segment_vec.dot(&segment_vec);
        let closest_point = 
            if t <= 0.0 {
                self.p1
            } else if t >= 1.0 {
                self.p2
            } else {
                self.p1.add(&segment_vec.mul(t))
            };
    
        // 2. Compute collision vector
        let collision_vector = particle.pos.sub(&closest_point);
        let distance = collision_vector.norm();
    
        // 3. Check if the point is on the correct side of the normal
        let side = (particle.pos.sub(&closest_point)).dot(&self.normal);

        // 4. Compute the collision velocity
        let collision_velocity = particle.vel.dot(&self.normal);

    
        if distance <= particle.radius && side >= 0.0 {
            Some(CollisionInfo {
                ke: (1.0/2.0) * particle.mass * collision_velocity * collision_velocity,
                normal: collision_vector.normalize(),
        })
        } else {
            None
        }
    } 
}

