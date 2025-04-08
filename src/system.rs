use std::cell;

use crate::grid::CellGrid;
use crate::particle::Particle;
use crate::shapes::Shape;

use dlt::dim_inv;
use dlt::dot;
use dlt::tensor::*;
use dlt::dimension::*;
use dlt::unit_mul;
use dlt::units::*;
use dlt::si::*;
use dlt::assert_dimension;
use dlt::dim_mul;
use dlt::dless;
use dlt::dimension;
use dlt::units;

//use rayon::prelude::*;

// First, let's define our custom error type
#[derive(Debug)]
pub enum ParticleError {
    OutOfBounds,
    // You can add more error types here as needed!
}

pub struct System {
    pub particles: Vec<Particle>,
    pub shapes: Vec<Shape>,
    pub height: Scalar<f32,Length>,
    pub width: Scalar<f32,Length>,

    // utilies optimisation
    pub cell_num: i32,
    cell_grid: CellGrid,
    cell_collisions: Vec<(usize, usize)>,

}

use std::marker::PhantomData;

const SPRING_COEFFICIENT: Scalar::<f32,dim_mul!(
    Mass, 
    (dim_mul!(
        (dim_inv!(Time)), 
        (dim_inv!(Time)))
    ))> = Scalar {
        data: [1000.0],
        _phantom: PhantomData
};

const DAMPING_COEFFICIENT: Scalar::<f32,dim_mul!(
    Mass, 
    (dim_inv!(Time))
    )> = Scalar {
        data: [1.0],
        _phantom: PhantomData
};

impl System {
    pub fn new(height: Scalar<f32,Length>, width: Scalar<f32,Length>, cnum: i32) -> System {
        let cell_grid = CellGrid::new(height.raw(), width.raw(), cnum, &Vec::new());
        let cell_collisions = cell_grid.get_cell_collisions();

        // print out the cell grid
        println!("{:?}", cell_grid);

        System {
            particles: Vec::new(),
            shapes: Vec::new(),
            height: height,
            width: width,

            cell_num: cnum,
            cell_grid: cell_grid,
            cell_collisions: cell_collisions,

        }
    }

    // Now let's modify the function to return a Result
    pub fn add(&mut self, particle: Particle) -> Result<(), ParticleError> {
        // check if the particle is inside the system
        if particle.pos.x() < Scalar::<f32,Length>::zero() || particle.pos.x() > self.width ||
        particle.pos.y() < Scalar::<f32,Length>::zero() || particle.pos.y() > self.height {
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

    pub fn update(&mut self, dt: Scalar<f32,Time>) {

        for particle in self.particles.iter_mut() {
            particle.update(dt);
        }

        // set the cells for the particles
        self.cell_grid.set_cells(&self.particles);
        //println!("{:?}", self.cell_grid);

        self.wall_collide();

        self.shape_collide();

        self.collide(dt);


    }

    pub fn collide(&mut self, dt: Scalar<f32,Time>) {
        let collisions = self.get_collisions();

        for (i, j) in collisions {
            let (f1, f2) = self.compute_collision_force(&self.particles[i as usize], &self.particles[j as usize]);
            
            // debug forces
            println!("f1 : {}, f2: {}", f1,f2);


            self.particles[i as usize].apply(f2,dt);
            self.particles[j as usize].apply(f1, dt);
        }
    }
    pub fn compute_collision_force(&self, p1: &Particle, p2: &Particle) -> (Vec2<f32,Force>, Vec2<f32,Force>) {
        let n = p1.pos - p2.pos;
        let distance = n.norm().cast::<f32>();
        
        // Early return for particles at same position
        if distance.mag() < (Scalar::<f64,_>::EPSILON) {
            return (Vec2::<f32,Force>::zero(), Vec2::<f32,Force>::zero());
        }

        // Normalize collision direction vector
        let normal = n / distance;

        // Compute particle overlap
        let overlap = (p1.radius + p2.radius) - distance;
        
        // No collision if no overlap
        if overlap <= Scalar::<f32,Length>::zero() {
            return (Vec2::<f32,Force>::zero(), Vec2::<f32,Force>::zero());
        }

        let v1 = p1.vel;
        let v2 = p2.vel;
        let rvel = v2 - v1;
        
        assert_dimension!(normal, Dimensionless);
        assert_dimension!(overlap, Length);
        assert_dimension!(rvel, Velocity);

        // Calculate forces
        let spring_force: Scalar<f32,Force> = SPRING_COEFFICIENT * overlap;
        let damping_force: Scalar<f32,Force> = DAMPING_COEFFICIENT * dot!(rvel, normal);
        let force = spring_force + damping_force;

        // Calculate force vectors using mass ratio to maintain conservation of momentum
        // The total force is distributed inversely proportional to the masses
        let total_mass = p1.mass + p2.mass;
        let p1_force_factor = p2.mass / total_mass;
        let p2_force_factor = p1.mass / total_mass;

        // Energy conservation correction
        // For elastic collisions, ensure the correct amount of energy is preserved
        // Adjust the force magnitude to conserve energy along the collision normal
        let force1 = normal.scale(-force * p1_force_factor);
        let force2 = normal.scale(force * p2_force_factor);

        (force1, force2)
    }

    pub fn wall_collide(&mut self) {
        // make particles bounce off walls

        for particle in self.particles.iter_mut() {
            if particle.pos.x() - particle.radius < Scalar::<f32,Length>::zero() {
                particle.vel.set_at(0,0, 0, -particle.vel.x());
                particle.pos.set_at(0,0, 0, particle.radius);
            }
            if particle.pos.x() + particle.radius > self.width {
                particle.vel.set_at(0,0, 0, -particle.vel.x());
                particle.pos.set_at(0,0, 0, self.width - particle.radius);
            }
            if particle.pos.y() - particle.radius < Scalar::<f32,Length>::zero() {
                particle.vel.set_at(0,1, 0, -particle.vel.y());
                particle.pos.set_at(0,1, 0, particle.radius);
            }
            if particle.pos.y() + particle.radius > self.height {
                particle.vel.set_at(0,1, 0, -particle.vel.y());
                particle.pos.set_at(0,1, 0, self.height - particle.radius);
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
                let distance = (p1.pos - p2.pos).norm().cast::<f32>();
                distance < p1.radius + p2.radius
            })
            .collect();

        collisions
    }


    // grid function (optimization)
    pub fn new_grid(&mut self) {
        self.cell_grid = CellGrid::new(
            self.height.raw(),
            self.width.raw(),
            self.cell_num,
            &self.particles,
        );
        self.cell_collisions = self.cell_grid.get_cell_collisions();

        println!("{:?}", self.cell_grid);
    }

    // reserved for future use
}
