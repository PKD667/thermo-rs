use raylib::prelude::*;
use rayon::vec;

use crate::linear::v2d;
use crate::particle::Particle;
use crate::system::System;

pub struct Renderer {
    pub width: i32,
    pub height: i32,
    pub scale: f64,

    // rendering stuff
    pub rl: RaylibHandle,
    pub th: RaylibThread,
}

impl Renderer {
    pub fn new(scale: f64, system: &System) -> Renderer {
        let (mut rl, th) = raylib::init()
            .size(
                system.width as i32 * scale as i32,
                system.height as i32 * scale as i32,
            )
            .title("Particle System")
            .build();

        Renderer {
            width: system.width as i32 * scale as i32,
            height: system.height as i32 * scale as i32,
            scale: scale,
            rl: rl,
            th: th,
        }
    }

    pub fn render(&mut self, mut system: &System) {
        let mut d = self.rl.begin_drawing(&self.th);
        d.clear_background(Color::WHITE);

    

        for p in system.particles.iter() {
            let x = p.pos.x * self.scale;
            let y = p.pos.y * self.scale;
            // color in blue, and darkness by absolute value of velocity
            //let color = Color::new(0, 0, 255, (p.vel.norm() * 40.0).clamp(0.0, 255.0) as u8);
            //d.draw_circle(x as i32, y as i32, (p.radius * self.scale) as f32, color);

            // drw density circles in blue ligher
            let color = Color::new(0, 0, 255, 50);
            d.draw_circle(x as i32, y as i32, (p.radius * self.scale * 10.0) as f32, color);

        }

        // draw shapes
        for shape in system.shapes.iter() {
            for segment in shape.segments.iter() {
                let p1 = segment.p1.mul(self.scale);
                let p2 = segment.p2.mul(self.scale);
                d.draw_line(
                    p1.x as i32,
                    p1.y as i32,
                    p2.x as i32,
                    p2.y as i32,
                    Color::BLACK,
                );
            }
        }
    }

    fn run(&mut self, mut system: &mut System,framerate: f64) {

        let mut frames: i64 = 0;
        let mut time: std::time::Instant = std::time::Instant::now();

        let mut avg_fps: f64 = 0.0;

        while !self.rl.window_should_close() {
            self.render(&system);

            // sleep to keep the framerate
            
            std::thread::sleep(std::time::Duration::from_secs_f64(1.0 / framerate));
            
            if frames / framerate as i64 > 1 {

                let elapsed = time.elapsed().as_secs_f64();

                avg_fps = frames as f64 / elapsed;
                frames = 0;
                time = std::time::Instant::now();
            }

            frames += 1;
        }

    }
}
