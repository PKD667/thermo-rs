
use crate::linear::v2d;
use crate::particle::Particle;
use crate::system::System;

use rand::Rng;

pub fn gen_particles(system: &mut System, n: i32, speed_factor: f64, mass: f32, radius: f32) {
    let mut rng = rand::thread_rng();

    let mut i = 0;
    loop {
        let vx = rng.gen_range(-1.0..1.0) * speed_factor;
        let vy = rng.gen_range(-1.0..1.0) * speed_factor;

        let mass = mass as f64;
        let radius = radius as f64;

        let x = rng.gen_range(radius + 0.1..system.width - radius - 0.1);
        let y = rng.gen_range(radius + 0.1..system.height - radius - 0.1);

        let p = Particle::new(v2d::new(x, y), v2d::new(vx, vy), mass, radius);

        let mut good = true;
        for p2 in system.particles.iter() {
            if p.dist(p2) < p.radius + p2.radius + 0.1 {
                good = false;
                break;
            }
        }

        if good {
            system.add(p);
            i += 1;
        }

        if i >= n {
            break;
        }
    }

    // update the system grid
    system.new_grid();

}

