use crate::particle::Particle;

pub struct CellGrid {
    pub height: f32,
    pub width: f32,

    cnum: i32,
    csize: f32,
    pub cells: Vec<Vec<i32>>,
}

impl CellGrid {
    pub fn set_cells(&mut self, particles: &Vec<Particle>) {
        for i in 0..self.cells.len() {
            self.cells[i].clear();
        }

        for i in 0..particles.len() {
            let cell = self.get_cell(particles[i].pos.raw_tuple());
            self.cells[cell as usize].push(i as i32);
        }
    }

    pub fn new(height: f32, width: f32, cnum: i32, particles: &Vec<Particle>) -> CellGrid {
        let mut cells = Vec::new();
        for _ in 0..cnum*cnum {
            cells.push(Vec::new());
        }

        let mut grid = CellGrid {
            height: height,
            width: width,
            cnum: cnum,
            csize: height / (cnum as f32),
            cells: cells,
        };

        grid.set_cells(particles);

        // print the grid matrix style 
        // with the number of particles in each cell
        for i in 0..grid.cells.len() {
            print!("{} ", grid.cells[i].len());
            if (i + 1) % grid.cnum as usize == 0 {
                println!();
            }
        }

        grid
    }

    pub fn cidx(&self, (x, y): (i32, i32)) -> i32 {
        // get a cell based on a matrix style index
        x + y * self.cnum
    }

    pub fn ccol(&self, idx: i32) -> i32 {
        // get the column of a cell based on its index
        idx % self.cnum
    }

    pub fn crow(&self, idx: i32) -> i32 {
        // get the row of a cell based on its index
        idx / self.cnum
    }

    pub fn get_cell_collisions(&self) -> Vec<(usize, usize)> {
        let mut cell_collisions = Vec::new();
        for i in 0..self.cells.len() {
            let (x, y) = (self.ccol(i as i32), self.crow(i as i32));

            for dx in -1..2 {
                for dy in -1..2 {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx >= 0 && nx < self.cnum && ny >= 0 && ny < self.cnum {
                        let idx = self.cidx((nx, ny));
                        cell_collisions.push((i, idx as usize));
                    }
                }
            }
        }
        cell_collisions
    }

    pub fn get_cell(&self, pos: (f32, f32)) -> i32 {
        let mut col = (pos.0 / self.csize).floor() as i32;
        let mut row = (pos.1 / self.csize).floor() as i32;
    
        // Clamp the column and row to valid indices.
        if col < 0 {
            col = 0;
        } else if col >= self.cnum {
            col = self.cnum - 1;
        }
    
        if row < 0 {
            row = 0;
        } else if row >= self.cnum {
            row = self.cnum - 1;
        }
    
        self.cidx((col, row))
    }
}

// implement debug
impl std::fmt::Debug for CellGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CellGrid: \n")?;
        for i in 0..self.cells.len() {
            write!(f, "{} ", self.cells[i].len())?;
            if (i + 1) % self.cnum as usize == 0 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}