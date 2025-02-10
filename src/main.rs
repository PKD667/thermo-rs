use thermo::math::v2d;
use thermo::particle::Particle;
use thermo::system::System;
use thermo::render::Renderer;


// random number generator
use rand::Rng;

const WIDTH: f64 = 200.0;
const HEIGHT: f64 = 100.0;

const N: i32 = 1000;

pub fn gen_particles(system: &mut System, n: i32, speed_factor: f64, mass: f32, radius: f32) {
    let mut rng = rand::thread_rng();

    let mut i = 0;
    loop {

        let vx = rng.gen_range(-1.0..1.0) * speed_factor;
        let vy = rng.gen_range(-1.0..1.0) * speed_factor;

        let mass = mass as f64;
        let radius = radius as f64;

        let x = rng.gen_range(radius + 0.1..WIDTH - radius - 0.1);
        let y = rng.gen_range(radius + 0.1..HEIGHT - radius - 0.1);

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


}

fn main() {
    let mut system = System::new(HEIGHT, WIDTH);

    gen_particles(&mut system, N, 20.0, 1.0, 1.0);

    let mut renderer = Renderer::new(4.0, &system);

    let dt = 1.0 / 200.0;
    let mut i = 0;

    let start_time = std::time::Instant::now();

    let mut tps = 0.0;

    let mut pressures: Vec<f64> = Vec::new();
    let mut temperatures: Vec<f64> = Vec::new();
    let mut kinetic_energies: Vec<f64> = Vec::new();

    system.modifier.force(v2d::new(0.0,-0.0));


    while !renderer.rl.window_should_close() {

        system.update(dt);

        renderer.render(&system);

    
        if tps > (1.0/dt) {
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
                wtr.write_record(&[pressures[i].to_string(), temperatures[i].to_string(), kinetic_energies[i].to_string()]).unwrap();
            }
            wtr.flush().unwrap();



        
            println!("Time: {}", system.measurer.get_time());

        }
    }
}
