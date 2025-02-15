// compute and analyse particle - segments collisions

use dlt::assert_dimension;
use dlt::dimension::Dimensionless;
use dlt::si::*;
use dlt::tensor::*;
use dlt::tensor::Vec2;
use dlt::dimension::Length;
use dlt::*;

use crate::particle::Particle;
use crate::physics::CollisionInfo;

pub struct Segment {
    pub p1: Vec2<Length>,
    pub p2: Vec2<Length>,

    pub normal: Vec2<Length>,
}


impl Segment {

    pub fn collision(&self, particle: &Particle) -> Option<CollisionInfo> {
        // 1. Find the closest point on the segment to our circle's center
        let segment_vec = self.p1 - self.p2;
        let point_vec = particle.pos - self.p1;
    
        // Project point onto segment
        let t = dot!(point_vec,segment_vec) * dot!(segment_vec,segment_vec).inv();
        let closest_point = self.p1 + segment_vec.scale(t);

        assert_dimension!(closest_point, Length);
        assert_dimension!(segment_vec, Length);
        assert_dimension!(t, Dimensionless);
    
        // 2. Compute collision vector
        let collision_vector = particle.pos - closest_point;
        let distance = collision_vector.norm();
    
        // 3. Check if the point is on the correct side of the normal
        let side = dot!(particle.pos - self.p1, self.normal);
        assert_dimension!(side, Area);

        // 4. Compute the collision velocity
        let collision_velocity = dot!(particle.vel, collision_vector) * distance.inv();
        assert_dimension!(collision_velocity, Velocity);
    
        if distance <= particle.radius && side >= Scalar::<Area>::zero() {
            Some(CollisionInfo {
                ke: (particle.mass * collision_velocity * collision_velocity)
                    .scale((0.5).dless()),
                normal: collision_vector,
        })
        } else {
            None
        }
    } 
}

