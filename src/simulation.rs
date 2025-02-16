
use crate::system::System;
use crate::render::Renderer;

// the simulation object simulates a System of particles
struct Simulation {
    pub system: System,
    pub renderer: Renderer,
}