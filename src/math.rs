/*
 * 2D Maths
*/
#[allow(non_camel_case_types)]
pub struct v2d {
    pub x: f64,
    pub y: f64,
}

impl v2d {
    pub fn new(x: f64, y: f64) -> v2d {
        v2d { x: x, y: y }
    }

    // copy trait
    pub fn clone(&self) -> v2d {
        v2d { x: self.x, y: self.y }
    }

    pub fn add(&self, other: &v2d) -> v2d {
        v2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &v2d) -> v2d {
        v2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn mul(&self, scalar: f64) -> v2d {
        v2d {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn div(&self, scalar: f64) -> v2d {
        v2d {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }

    pub fn dot(&self, other: &v2d) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &v2d) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dist(&self, other: &v2d) -> f64 {
        let v2 = self.sub(other);
        v2.norm()
    }
}

/*
 * 3D Maths
*/
#[allow(non_camel_case_types)]
pub struct v3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
