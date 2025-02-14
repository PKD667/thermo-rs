use std::cell;

use crate::grid::CellGrid;
use crate::particle::Particle;
use crate::shapes::Shape;

use dlt::tensor::*;
use dlt::dimension::*;
use dlt::units::*;
use dlt::dot;

//use rayon::prelude::*;

pub struct System {
    pub particles: Vec<Particle>,
    pub shapes: Vec<Shape>,
    pub height: Scalar<Length>,
    pub width: Scalar<Length>,

    // utilies optimisation
    cell_grid: CellGrid,
    cell_collisions: Vec<(usize, usize)>,

}

const DEFAULT_CSIZE: i32 = 10;

// First, let's define our custom error type
#[derive(Debug)]
pub enum ParticleError {
    OutOfBounds,
    // You can add more error types here as needed!
}


impl System {
    pub fn new(height: Scalar<Length>, width: Scalar<Length>) -> System {
        let csize = DEFAULT_CSIZE;
        let cell_grid = CellGrid::new(height.raw() as i64, width.raw() as i64, csize, &Vec::new());
        let cell_collisions = cell_grid.get_cell_collisions();

        System {
            particles: Vec::new(),
            shapes: Vec::new(),
            height: height,
            width: width,

            cell_grid: cell_grid,
            cell_collisions: cell_collisions,

        }
    }

    // Now let's modify the function to return a Result
    pub fn add(&mut self, particle: Particle) -> Result<(), ParticleError> {
        // check if the particle is inside the system
        if particle.pos.x() < Scalar::<Length>::zero() || particle.pos.x() > self.width ||
        particle.pos.y() < Scalar::<Length>::zero() || particle.pos.y() > self.height {
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

    pub fn update(&mut self, dt: Scalar<Time>) {

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
            let (v1, v2) = self.apply_collision_equation(&self.particles[i as usize], &self.particles[j as usize]);
            self.particles[i as usize].vel = v1;
            self.particles[j as usize].vel = v2;
        }
    }

    pub fn apply_collision_equation(&self, p1: &Particle, p2: &Particle) -> (Vec2<Velocity>, Vec2<Velocity>) {
        let m1 = p1.mass;
        let m2 = p2.mass;

        let n = p1.pos - p2.pos;
        let i_norm = n.norm().inv();
        let n = n.scale(i_norm);

        let v1 = p1.vel;
        let v2 = p2.vel;

        
        let v1n = n.scale(dot!(v1, n));
        let v1t = v1 - v1n;
        let v2n = n.scale(dot!(v2, n));
        let v2t = v2 - v2n;

        let inv_ms = (m1 + m2).inv();

        let v1f = 
            v1n * (
                (m1 - m2) 
                    * inv_ms
                ) + 
                v2n * (
                    (m2+m2) * inv_ms
                );
                
        let v2f = 
            v2n * (
                (m2 - m1) 
                    * inv_ms
                ) + 
                v1n * (
                    (m1+m1) * inv_ms
                );

        let v1 = v1f + v1t;
        let v2 = v2f + v2t;

        (v1, v2)
    }

    pub fn wall_collide(&mut self) {
        // make particles bounce off walls

        for particle in self.particles.iter_mut() {
            if particle.pos.x() - particle.radius < Scalar::<Length>::zero() {
                particle.vel.set_at(0, 0, -particle.vel.x());
                particle.pos.set_at(0, 0, particle.radius);
            }
            if particle.pos.x() + particle.radius > self.width {
                particle.vel.set_at(0, 0, -particle.vel.x());
                particle.pos.set_at(0, 0, self.width - particle.radius);
            }
            if particle.pos.y() - particle.radius < Scalar::<Length>::zero() {
                particle.vel.set_at(0, 1, -particle.vel.y());
                particle.pos.set_at(0, 1, particle.radius);
            }
            if particle.pos.y() + particle.radius > self.height {
                particle.vel.set_at(0, 1, -particle.vel.y());
                particle.pos.set_at(0, 1, self.height - particle.radius);
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
            .into_iter()
            .filter(|(i, j)| {
                let p1 = &self.particles[*i as usize];
                let p2 = &self.particles[*j as usize];
                let distance = (p1.pos - p2.pos).norm();
                distance < p1.radius + p2.radius
            })
            .collect();

        collisions
    }


    // grid function (optimization)
    pub fn new_grid(&mut self) {
        self.cell_grid = CellGrid::new(
            self.height.raw() as i64,
            self.width.raw() as i64,
            DEFAULT_CSIZE,
            &self.particles,
        );
        self.cell_collisions = self.cell_grid.get_cell_collisions();
    }



    // reserved for future use
}
