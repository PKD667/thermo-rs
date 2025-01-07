
use raylib::prelude::*;

use crate::math::v2d;
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

        let (mut rl,th) = raylib::init()
            .size(system.width as i32 * scale as i32, system.height as i32 * scale as i32)
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
            let color = Color::new(
                0,
                0,
                255,
                (p.vel.norm() * 40.0).clamp(0.0, 255.0) as u8
            );
            d.draw_circle(x as i32, y as i32, (p.radius * self.scale) as f32, color);
        }
    }
}