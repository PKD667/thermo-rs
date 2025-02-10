use std::cell;

use crate::math::v2d;
use crate::particle::Particle;

use crate::measure::Measurer;

use crate::tools::Modifier;

use rayon::prelude::*;





struct CellGrid {
    pub height: i64,
    pub width: i64,
    
    pub csize: i32,
    pub cells: Vec<Vec<i32>>,

    // stuff cached for optimization
    pub cx: i32,
    pub cy: i32,
}

impl CellGrid {

    pub fn set_cells(&mut self, particles: &Vec<Particle>) {
        for i in 0..self.cells.len() {
            self.cells[i].clear();
        }

        for i in 0..particles.len() {
            let cell = self.get_cell(particles[i].pos.clone());
            self.cells[cell as usize].push(i as i32);
        }

    }

    pub fn new (height: i64, width: i64, csize: i32,particles: &Vec<Particle>) -> CellGrid {
        let cx = ((width as i32 / csize) + 1) as i32;
        let cy = ((height as i32 / csize) + 1) as i32;

        let mut cells = Vec::new();
        for _ in 0..cx * cy {
            cells.push(Vec::new());
        }

        let mut grid = CellGrid {
            height: height,
            width: width,
            csize: csize,
            cells: cells,
            cx: cx,
            cy: cy
        };

        grid.set_cells(particles);

        grid
    }

    pub fn cidx(&self,(x, y): (i32, i32)) -> i32 {
        // get a cell based on a matrix style index
        x + y * self.cx
    }

    pub fn ccol(&self, idx: i32) -> i32 {
        // get the column of a cell based on its index
        idx % self.cx
    }

    pub fn crow(&self, idx: i32) -> i32 {
        // get the row of a cell based on its index
        idx / self.cx
    }



    pub fn get_cell(&self, pos: v2d) -> i32 {

        let x = ((pos.x as i32) / self.csize) as i32;
        let y = ((pos.y as i32) / self.csize) as i32;

        x + (y * self.cx) as i32
    }

}




pub struct System {
    pub particles: Vec<Particle>,
    pub height: f64,
    pub width: f64,
    csize: i32,
    
    // measurer
    pub measurer: Measurer,

    // tools shit
    pub modifier: Modifier

}

const DEFAULT_CSIZE: i32 = 5 ;

impl System {
    pub fn new(height: f64, width: f64) -> System {
        System {
            particles: Vec::new(),
            height: height,
            width: width,
            csize: DEFAULT_CSIZE,
            measurer: Measurer::new(),
            modifier: Modifier {Q:0.0,F: v2d::new(0.0,0.0)}
        }
    }

    pub fn add(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn update(&mut self, dt: f64) {

        self.measurer.record_time(dt);

        self.wall_collide();

        self.apply_general_force(self.modifier.F.clone());

        self.collide();

        for particle in self.particles.iter_mut() {
            particle.update(dt);
        }
    }

    pub fn collide(&mut self) {

        let collisions = self.get_collisions();

        for (i, j) in collisions {
            let (v1, v2) = self.apply_collision_equation(&self.particles[i as usize], &self.particles[j as usize]);
            self.particles[i as usize].vel = v1;
            self.particles[j as usize].vel = v2;

            // push particles apart by a small amount
            let n = self.particles[i as usize].pos.sub(&self.particles[j as usize].pos);
            let n = n.div(n.norm());
            let push = n.mul(0.001);
            self.particles[i as usize].pos = self.particles[i as usize].pos.add(&push);
            self.particles[j as usize].pos = self.particles[j as usize].pos.sub(&push);

        }

    }

    pub fn apply_general_force(&mut self, force: v2d) {
        for particle in self.particles.iter_mut() {
            particle.vel = particle.vel.add(&force);
        }
    }


    pub fn apply_collision_equation(&self, p1: &Particle, p2: &Particle) -> (v2d, v2d) {
        let m1 = p1.mass;
        let m2 = p2.mass;
        let n = p1.pos.sub(&p2.pos);
        let n = n.div(n.norm());
        let v1 = p1.vel.clone();
        let v2 = p2.vel.clone();
        let v1n = n.mul(v1.dot(&n));
        let v1t = v1.sub(&v1n);
        let v2n = n.mul(v2.dot(&n));
        let v2t = v2.sub(&v2n);

        let v1f = v1n.mul((m1 - m2)/(m1 + m2)).add(&v2n.mul(2.0*m2/(m1 + m2)));
        let v2f = v2n.mul((m2 - m1)/(m1 + m2)).add(&v1n.mul(2.0*m1/(m1 + m2)));
        let v1 = v1f.add(&v1t);
        let v2 = v2f.add(&v2t);

        let loss = 0.01;

        (v1.mul(1.0 - loss), v2.mul(1.0 - loss))

    }

    pub fn wall_collide(&mut self) {
        for particle in self.particles.iter_mut() {
            let mut collided = false;



            let factor = self.modifier.Q / particle.mass;

    
            // Check collision on the x-axis
            if particle.pos.x < particle.radius {
                particle.pos.x = particle.radius;
                particle.vel.x = -particle.vel.x * (particle.vel.x.abs() / (particle.vel.x.abs() + factor));
                collided = true;
            } else if particle.pos.x + particle.radius > self.width {
                particle.pos.x = self.width - particle.radius;
                particle.vel.x = -particle.vel.x * (particle.vel.x.abs() / (particle.vel.x.abs() + factor));
                collided = true;
            }
    
            // Check collision on the y-axis
            if particle.pos.y < particle.radius {
                particle.pos.y = particle.radius;
                particle.vel.y = -particle.vel.y * (particle.vel.y.abs() / (particle.vel.y.abs() + factor));
                collided = true;
            } else if particle.pos.y + particle.radius > self.height {
                particle.pos.y = self.height - particle.radius;
                particle.vel.y = -particle.vel.y * (particle.vel.y.abs() / (particle.vel.y.abs() + factor));
                collided = true;
            }
    
            if collided {
                self.measurer.record_wall_hit(&particle);
                // Push particles apart by a small amount
                let push = particle.vel.mul(0.01);
                particle.pos = particle.pos.add(&push);

                self.modifier.Q -= factor;
            }


        }
    }


    // ...existing code...
    pub fn get_collisions(&mut self) -> Vec<(i32, i32)> {

        let grid = self.get_cells();

        let mut cell_collisions = Vec::new();
        for i in 0..grid.cells.len() {
            let (x, y) = (grid.ccol(i as i32), grid.crow(i as i32));

            for dx in -1..2 {
                for dy in -1..2 {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx >= 0 && nx < grid.cx && ny >= 0 && ny < grid.cy {
                        let idx = grid.cidx((nx, ny));
                        cell_collisions.push((i, idx as usize));
                    }
                }
            }

        }

        let mut all_collisions = Vec::new();
        for (i, j) in cell_collisions {
            let vec_a = &grid.cells[i];
            let vec_b = &grid.cells[j];
            let mut pair_collisions = vec_a
                .iter()
                .flat_map(|x| {
                    vec_b
                        .iter()
                        .filter(move |y| x < y)
                        .map(move |y| (*x, *y))
                })
                .collect();
            all_collisions.append(&mut pair_collisions);
        }
        


        // ...existing code...
        let filtered_collisions = all_collisions
            .par_iter()
            .cloned()
            .filter(|(i, j)| {
                let p1 = &self.particles[*i as usize];
                let p2 = &self.particles[*j as usize];
                p1.dist(p2) < p1.radius + p2.radius
            })
            .collect::<Vec<(i32, i32)>>();
        
        filtered_collisions
        // ...existing code...

    }

    // partition the particle list into cells
    fn get_cells(&mut self) -> CellGrid {

        CellGrid::new(
            self.height as i64, 
            self.width as i64, 
            self.csize, 
            &self.particles
        )
    }




    // reserved for future use
}
