use std::cell;

use crate::grid::CellGrid;
use crate::linear::v2d;
use crate::particle::Particle;
use crate::shapes::Shape;

use crate::measure::Measurer;

use rayon::prelude::*;

pub struct System {
    pub particles: Vec<Particle>,
    pub shapes: Vec<Shape>,
    pub height: f64,
    pub width: f64,

    // utilies optimisation
    cell_grid: CellGrid,
    cell_collisions: Vec<(usize, usize)>,

    // measurer
    pub measurer: Measurer,
}

const DEFAULT_CSIZE: i32 = 10;

// First, let's define our custom error type
#[derive(Debug)]
pub enum ParticleError {
    OutOfBounds,
    // You can add more error types here as needed!
}


impl System {
    pub fn new(height: f64, width: f64, shapes: Vec<Shape>) -> System {
        let csize = DEFAULT_CSIZE;
        let cell_grid = CellGrid::new(height as i64, width as i64, csize, &Vec::new());
        let cell_collisions = cell_grid.get_cell_collisions();

        System {
            particles: Vec::new(),
            shapes: shapes,
            height: height,
            width: width,

            cell_grid: cell_grid,
            cell_collisions: cell_collisions,

            measurer: Measurer::new(),
        }
    }

    // Now let's modify the function to return a Result
    pub fn add(&mut self, particle: Particle) -> Result<(), ParticleError> {
        // check if the particle is inside the system
        if particle.pos.x < 0.0 || particle.pos.x > self.width ||
        particle.pos.y < 0.0 || particle.pos.y > self.height {
            return Err(ParticleError::OutOfBounds);
        }

        self.particles.push(particle);
        Ok(())
    }

    pub fn add_shape(&mut self, shape: Shape) {
        // check if (all) the shape is inside the system
        // iterate over segments in

        self.shapes.push(shape);
    }

    pub fn update(&mut self, dt: f64) {

        // measure tools
        self.measurer.record_time(dt);

        // set the cells for the particles
        self.cell_grid.set_cells(&self.particles);

        self.wall_collide();

        self.shape_collide();

        self.collide();

        for particle in self.particles.iter_mut() {
            particle.update(dt);
        }
    }

    pub fn collide(&mut self) {
        let collisions = self.get_collisions();
        // print the number of collisions


        for (i, j) in collisions {
            let (p1, p2) = unsafe {
                // Get mutable references to both particles at once
                (
                    &mut *(&mut self.particles[i as usize] as *mut _),
                    &mut *(&mut self.particles[j as usize] as *mut _),
                )
            };

            // Calculate velocities
            let (v1, v2) = self.apply_collision_equation(p1, p2);

            // Calculate position adjustment in one go
            let vd = p1.pos.sub(&p2.pos);
            let delta = vd.mul((p1.radius + p2.radius - vd.norm() + 0.0001) / 2.0);

            // Update everything at once
            p1.vel = v1;
            p2.vel = v2;
            p1.pos = p1.pos.add(&delta);
            p2.pos = p2.pos.add(&delta.mul(-1.0));
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

        let v1f = v1n
            .mul((m1 - m2) / (m1 + m2))
            .add(&v2n.mul(2.0 * m2 / (m1 + m2)));
        let v2f = v2n
            .mul((m2 - m1) / (m1 + m2))
            .add(&v1n.mul(2.0 * m1 / (m1 + m2)));
        let v1 = v1f.add(&v1t);
        let v2 = v2f.add(&v2t);

        (v1, v2)
    }

    pub fn wall_collide(&mut self) {
        // make particles bounce off walls

        for particle in self.particles.iter_mut() {
            if particle.pos.x - particle.radius < 0.0 {
                particle.vel.x = -particle.vel.x;
                particle.pos.x = particle.radius;
            }
            if particle.pos.x + particle.radius > self.width {
                particle.vel.x = -particle.vel.x;
                particle.pos.x = self.width - particle.radius;
            }
            if particle.pos.y - particle.radius < 0.0 {
                particle.vel.y = -particle.vel.y;
                particle.pos.y = particle.radius;
            }
            if particle.pos.y + particle.radius > self.height {
                particle.vel.y = -particle.vel.y;
                particle.pos.y = self.height - particle.radius;
            }
        }
    }

    pub fn shape_collide(&mut self) {
        for particle in self.particles.iter_mut() {
            for shape in self.shapes.iter_mut() {
                shape.collide(particle);
            }
        }
    }


    // compute all particle to particle collisions
    pub fn get_collisions(&mut self) -> Vec<(i32, i32)> {
        
        let mut possible_collisions = Vec::new();
        
        for (ci, cj) in self.cell_collisions.iter() {
            // appends all the possible combinations of particles in the two cells
            // make sure that the particles are not the same
            for i in self.cell_grid.cells[*ci].iter() {
                for j in self.cell_grid.cells[*cj].iter() {
                    if i < j {
                        possible_collisions.push((*i, *j));
                    }
                }
            }
        }


        // filter out the collisions that are actually happening
        let collisions: Vec<(i32, i32)> = possible_collisions
            .into_par_iter()
            .filter(|(i, j)| {
                let p1 = &self.particles[*i as usize];
                let p2 = &self.particles[*j as usize];
                let distance = p1.pos.sub(&p2.pos).norm();
                distance < p1.radius + p2.radius
            })
            .collect();

        collisions
    }


    // grid function (optimization)
    pub fn new_grid(&mut self) {
        self.cell_grid = CellGrid::new(
            self.height as i64,
            self.width as i64,
            DEFAULT_CSIZE,
            &self.particles,
        );
        self.cell_collisions = self.cell_grid.get_cell_collisions();
    }



    // reserved for future use
}
