
use raylib::prelude::*;

use crate::particle::Particle;
use crate::system::System;

pub struct Renderer {
    pub width: i32,
    pub height: i32,
    pub scale: f32,

    // rendering stuff
    pub rl: RaylibHandle,
    pub th: RaylibThread,
}

impl Renderer {
    pub fn new(scale: f32, system: &System) -> Renderer {

        let (rl,th) = raylib::init()
            .size((system.width.raw() * scale) as i32, (system.height.raw() * scale) as i32)
            .title("Particle System")
            .build();

        let width = (system.width.raw() * scale) as i32;
        let heigth  = (system.width.raw() * scale) as i32;

        Renderer {
            width: width,
            height: heigth,
            scale: scale,
            rl: rl,
            th: th,
        }
    }

    pub fn render(&mut self, system: &System) {
        let mut d = self.rl.begin_drawing(&self.th);
        d.clear_background(Color::BLACK);

        for p in system.particles.iter() {
            let x = p.pos.x().raw() * self.scale;
            let y = p.pos.y().raw() * self.scale;
            // color in blue, and darkness by absolute value of velocity
            let color = Color::new(
                0,
                0,
                255,
                255
            );

            //println!("x: {}, y: {}, r: {}", x, y, p.radius.raw() * self.scale);

            d.draw_circle(x as i32, y as i32, (p.radius.raw() * self.scale) as f32, color);
        }
    }
}