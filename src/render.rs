
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

struct Circle {
    x: i32,
    y: i32,
    radius: f32,
}

impl Circle {
    pub fn new(x: i32, y: i32, radius: f32) -> Circle {
        Circle { x, y, radius }
    }
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

    fn get_circles(&self, system: &System) -> Vec<Circle> {
        let mut circles = Vec::new();
        for p in system.particles.iter() {
            let x = (p.pos.x().raw() * self.scale) as i32;
            let y = (p.pos.y().raw() * self.scale) as i32;
            let radius = (p.radius.raw() * self.scale) as f32;
            circles.push(Circle::new(x, y, radius));
        }
        circles
    }

    pub fn render(&mut self, system: &System) {
        let mut d = self.rl.begin_drawing(&self.th);
        d.clear_background(Color::BLACK);
    
        // Cache the scale and constant color outside the loop.
        let scale = self.scale;
        let color = Color::new(0, 0, 255, 255);

        // draw the optimization grid in red
        let csize = self.height / (system.cell_num);
        for i in 0..system.cell_num {
            for j in 0..system.cell_num {
                d.draw_rectangle_lines(i * csize, j * csize, csize, csize, Color::new(255,0,0,50));
            }
        }
    
        for p in system.particles.iter() {
            // Calculate x, y and radius once.
            let x = (p.pos.x().raw() * scale) as i32;
            let y = (p.pos.y().raw() * scale) as i32;
            let radius = (p.radius.raw() * scale) as f32;
    
            d.draw_circle(x, y, radius, color);
        }
    }
}