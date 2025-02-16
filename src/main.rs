#![feature(generic_const_exprs)]
#![feature(trivial_bounds)]
#![feature(generic_arg_infer)]

use dlt::*;
use thermo::particle::Particle;
use thermo::system::System;
use thermo::render::Renderer;

use dlt::tensor::*;
use dlt::dimension::*;
use dlt::units::*;
use dlt::si::*;

// random number generator
use rand::Rng;

fn gen_particles(n: usize, height: Scalar<Length>, width: Scalar<Length>) -> Vec<Particle> {
    let mut rng = rand::thread_rng();
    let mut particles: Vec<Particle> = Vec::new();

    let desired_particles = n;
    while particles.len() < desired_particles {
        let pos = Vec2::<Length>::new::<base_unit_dim!(Length)>([
            rng.gen_range(0.0..height.raw()),
            rng.gen_range(0.0..width.raw()),
        ]);
        let r = Scalar::<Length>::from::<Meter>(rng.gen_range(1.0..10.0));

        let vel = Vec2::<Velocity>::new::<unit_div!(Meter,Second)>([
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-100.0..100.0),
        ]);
        let m = Scalar::<Mass>::from::<Kilogram>(rng.gen_range(1.0..10.0));

        let candidate = Particle::new(pos, vel, m, r);
        let mut valid = true;

        // Check the candidate against all previously added particles.
        for existing in &particles {
            let (cand_x, cand_y) = candidate.pos.raw_tuple();
            let (ex_x, ex_y) = existing.pos.raw_tuple();
            let dx = cand_x - ex_x;
            let dy = cand_y - ex_y;
            let dist_sq = dx * dx + dy * dy;
            // Minimum allowed distance: one-third of the sum of the radii.
            let min_dist = (candidate.radius.raw() + existing.radius.raw()) + (candidate.radius.raw() + existing.radius.raw()) / 3.0;
            if dist_sq < min_dist * min_dist {
                valid = false;
                break;
            }
        }

        if valid {
            particles.push(candidate);
        }
    }

    particles
}

fn main() {
    let height = Scalar::<Length>::from::<Meter>(200.0);
    let width = Scalar::<Length>::from::<Meter>(200.0);

    println!("Height: {}m", height);
    println!("Width: {}m", width);

    let mut system = System::new(height, width, 10);

    // one particle
    let mut rng = rand::thread_rng();


    let particles = gen_particles(10, height, width);
    for p in particles {
        system.add(p);
    }


    let scale: STYPE = 4.0;
    println!("Scale: {}", scale);
    let mut renderer = Renderer::new(scale, &system);

    let dt = Scalar::<Time>::from::<Millisecond>(1.0);

    let mut i = 0;
    let start_time = std::time::Instant::now();
    let mut tps = 0.0;

    let mut total_update_duration = std::time::Duration::new(0, 0);
    let mut total_render_duration = std::time::Duration::new(0, 0);

    while !renderer.rl.window_should_close() {
        let update_start = std::time::Instant::now();
        system.update(dt);
        let update_duration = update_start.elapsed();
        total_update_duration += update_duration;

        let render_start = std::time::Instant::now();
        renderer.render(&system);
        let render_duration = render_start.elapsed();
        total_render_duration += render_duration;

        if tps > (dt.inv().raw() as f64) {
            //std::thread::sleep(std::time::Duration::from_millis(
            //    dt.scale((1000.0).dless()).get_at::<Millisecond>(0,0) as u64)
            //);
        }

        i += 1;
        
        tps = i as f64 / start_time.elapsed().as_secs_f64();
        if i % 1000 == 0 {
            let mean_update_duration = total_update_duration / 1000;
            let mean_render_duration = total_render_duration / 1000;
            println!("Mean system.update() duration: {:?}", mean_update_duration);
            println!("Mean renderer.render() duration: {:?}", mean_render_duration);
            println!("TPS: {}", tps);

            // Reset the total durations
            total_update_duration = std::time::Duration::new(0, 0);
            total_render_duration = std::time::Duration::new(0, 0);
        }
    }
}