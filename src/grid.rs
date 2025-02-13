use crate::linear::v2d;
use crate::particle::Particle;

pub struct CellGrid {
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

    pub fn new(height: i64, width: i64, csize: i32, particles: &Vec<Particle>) -> CellGrid {
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
            cy: cy,
        };

        grid.set_cells(particles);

        // print the grid matrix style 
        // with the number of particles in each cell
        for i in 0..grid.cells.len() {
            print!("{} ", grid.cells[i].len());
            if (i + 1) % grid.cx as usize == 0 {
                println!();
            }
        }

        grid
    }

    pub fn cidx(&self, (x, y): (i32, i32)) -> i32 {
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

    pub fn get_cell_collisions(&self) -> Vec<(usize, usize)> {
        let mut cell_collisions = Vec::new();
        for i in 0..self.cells.len() {
            let (x, y) = (self.ccol(i as i32), self.crow(i as i32));

            for dx in -1..2 {
                for dy in -1..2 {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx >= 0 && nx < self.cx && ny >= 0 && ny < self.cy {
                        let idx = self.cidx((nx, ny));
                        cell_collisions.push((i, idx as usize));
                    }
                }
            }
        }
        cell_collisions
    }

    pub fn get_cell(&self, pos: v2d) -> i32 {
        let x = ((pos.x as i32) / self.csize) as i32;
        let y = ((pos.y as i32) / self.csize) as i32;

        x + (y * self.cx) as i32
    }
}
