use thermo::math::v2d;
use thermo::particle::Particle;
use thermo::render::Renderer;
use thermo::shapes::Shape;
use thermo::system::System;

// random number generator
use rand::Rng;

const WIDTH: f64 = 400.0;
const HEIGHT: f64 = 500.0;


fn main() {
    let shapes = vec![];

    let mut system = System::new(HEIGHT, WIDTH, shps);

    gen_particles(&mut system, 300, 100.0, 1.0, 5.0);


    while !renderer.rl.window_should_close() {
        system.update(dt);

        renderer.render(&system);

        if tps > (1.0 / dt) {
            std::thread::sleep(std::time::Duration::from_millis((dt * 1000.0) as u64));
        }

        i += 1;

        tps = i as f64 / start_time.elapsed().as_secs_f64();
        if i % 100 == 0 {
            println!("TPS: {}", tps);

            // print measurements
            let p = system.measurer.get_pressure(&system);
            let t = system.measurer.get_temperature(&system);
            let k = system.measurer.get_kinetic_energy(&system);

            println!("Pressure: {}", p);
            println!("Temperature: {}", t);
            println!("Kinetic Energy: {}", k);

            pressures.push(p);
            temperatures.push(t);
            kinetic_energies.push(k);

            // save in a csv file
            let mut wtr = csv::Writer::from_path("data.csv").unwrap();
            for i in 0..pressures.len() {
                wtr.write_record(&[
                    pressures[i].to_string(),
                    temperatures[i].to_string(),
                    kinetic_energies[i].to_string(),
                ])
                .unwrap();
            }
            wtr.flush().unwrap();

            println!("Time: {}", system.measurer.get_time());
        }

        // gravity
        for p in system.particles.iter_mut() {
            p.vel.y += 0.01;
        }
    }
}
