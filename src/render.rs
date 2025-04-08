
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

            // draw nice velocity vector arrow
            let vel_x = (p.vel.x().raw() * scale) as i32;
            let vel_y = (p.vel.y().raw() * scale) as i32;
            // draw a pointy arrow good looking
            draw_arrow(
                &mut d,
                Vector2::new(x as f32, y as f32),
                Vector2::new((x + vel_x) as f32, (y + vel_y) as f32),
                Color::new(255, 0, 0, 255),
            );

            // draw acc in green
            let acc_x = (p.acc.x().raw() * scale) as i32;
            let acc_y = (p.acc.y().raw() * scale) as i32;
            // draw a pointy arrow good looking
            draw_arrow(
                &mut d,
                Vector2::new(x as f32, y as f32),
                Vector2::new((x + acc_x) as f32, (y + acc_y) as f32),
                Color::new(0, 255, 0, 255),
            );

        }
    }
}

fn draw_arrow(d: &mut RaylibDrawHandle, start: Vector2, end: Vector2, color: Color) {
    // Calculate direction vector
    let vel_x = end.x - start.x;
    let vel_y = end.y - start.y;
    let arrow_len = (vel_x.powi(2) + vel_y.powi(2)).sqrt();
    
    // Don't draw arrows that are too small
    if arrow_len < 1.0 {
        return;
    }
    
    // Draw the main line with increased thickness
    d.draw_line_ex(start, end, 3.0, color);  // Increased from 2.0 to 3.0

    // Arrow head parameters - significantly increased for better visibility
    let tip_scale = 0.1;    // Increased from 0.1
    let side_width = 0.05;  // Increased from 0.05
    
    // Normalized direction vector
    let dir_x = vel_x / arrow_len;
    let dir_y = vel_y / arrow_len;
    
    // Base of the arrow head
    let base_x = end.x - dir_x * (arrow_len * tip_scale);
    let base_y = end.y - dir_y * (arrow_len * tip_scale);

    // Perpendicular direction for the arrow head sides
    let perp_x = -dir_y;
    let perp_y = dir_x;

    // Calculate points for the arrow head
    let left_x = base_x + perp_x * (arrow_len * side_width);
    let left_y = base_y + perp_y * (arrow_len * side_width);
    let right_x = base_x - perp_x * (arrow_len * side_width);
    let right_y = base_y - perp_y * (arrow_len * side_width);

    // Create point vectors for the arrow head
    let tip = Vector2::new(end.x, end.y);
    let left = Vector2::new(left_x, left_y);
    let right = Vector2::new(right_x, right_y);
    
    // Since draw_triangle doesn't seem to work, let's try an alternative approach
    // for a filled triangle using polygon drawing
    let points = [tip, left, right];
    d.draw_triangle_fan(&points, color);
    
    // Also keep the triangle lines which are working
    d.draw_triangle_lines(tip, left, right, color);
}