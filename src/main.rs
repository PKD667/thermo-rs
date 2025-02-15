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

fn main() {
    let height = Scalar::<Length>::from::<Ångström>(1000.0);
    let width = Scalar::<Length>::from::<Ångström>(1000.0);

    println!("Height: {} Ångström", height);
    println!("Width: {} Ångström", width);

    let mut system = System::new(height, width, 20);

    // one particle
    let mut rng = rand::thread_rng();

    for _ in 0..500 {
        let pos = Vec2::<Length>::new::<base_unit_dim!(Length)>(
            [rng.gen_range(0.0..height.raw()), rng.gen_range(0.0..height.raw())]
        );
        let r = Scalar::<Length>::from::<Ångström>(rng.gen_range(1.0..10.0));

        let vel = Vec2::<Velocity>::new::<unit_div!(Ångström,Second)>(
            [rng.gen_range(-10.0..10.0), rng.gen_range(-100.0..100.0)]
        );
        let m = Scalar::<Mass>::from::<Dalton>(rng.gen_range(1.0..10.0));

        let p = Particle::new(pos, vel, m, r);
        system.add(p).unwrap();
    }

    let scale: STYPE = <base_unit_dim!(Length) as Unit>::ratio::<Ångström>();
    println!("Scale: {}", scale);
    let mut renderer = Renderer::new(scale, &system);

    let dt = Scalar::<Time>::from::<Millisecond>(1.0);

    let mut i = 0;
    let start_time = std::time::Instant::now();
    let mut tps = 0.0;

    while !renderer.rl.window_should_close() {
        let update_start = std::time::Instant::now();
        system.update(dt);
        let update_duration = update_start.elapsed();
        //println!("system.update() took: {:?}", update_duration);

        let render_start = std::time::Instant::now();
        renderer.render(&system);
        let render_duration = render_start.elapsed();
        //println!("renderer.render() took: {:?}", render_duration);

        if tps > (dt.inv().raw() as f64) {
            //std::thread::sleep(std::time::Duration::from_millis(
            //    dt.scale((1000.0).dless()).get_at::<Millisecond>(0,0) as u64)
            //);
        }

        i += 1;
        
        tps = i as f64 / start_time.elapsed().as_secs_f64();
        if i % 100 == 0 {
            println!("TPS: {}", tps);
        }
    }
}