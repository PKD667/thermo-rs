use std::ops::{Add, Sub, Mul, Div};
use uom::si::{Quantity, SI};

pub type LengthV2D = v2d<uom::si::length::Dimension>;
pub type ForceV2D = v2d<uom::si::force::Dimension>;
pub type VelocityV2D = v2d<uom::si::velocity::Dimension>;



/// A 2D vector whose underlying numeric type is always f64, but dimension remains generic.
#[allow(non_camel_case_types)]
pub struct v2d<D: uom::si::Dimension + ?Sized> {
    pub x: Quantity<D, SI<f64>, f64>,
    pub y: Quantity<D, SI<f64>, f64>,
}

impl<D: uom::si::Dimension +?Sized> v2d<D> {
    pub fn new(x: Quantity<D, SI<f64>, f64>, y: Quantity<D, SI<f64>, f64>) -> Self {
        v2d { x, y }
    }

    pub fn add(&self, other: &Self) -> Self
    where
        Quantity<D, SI<f64>, f64>: Add<Output = Quantity<D, SI<f64>, f64>>,
    {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Self) -> Self
    where
        Quantity<D, SI<f64>, f64>: Sub<Output = Quantity<D, SI<f64>, f64>>,
    {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn scalar_mul<Factor, OutDim>(&self, factor: Factor) -> v2d<OutDim>
    where
        Factor: Copy,
        OutDim: uom::si::Dimension+ ?Sized,
        Quantity<D, SI<f64>, f64>: Mul<Factor, Output = Quantity<OutDim, SI<f64>, f64>>,
    {
        v2d {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    pub fn scalar_div<Factor, OutDim>(&self, factor: Factor) -> v2d<OutDim>
    where
        Factor: Copy,
        OutDim: uom::si::Dimension,
        Quantity<D, SI<f64>, f64>: Div<Factor, Output = Quantity<OutDim, SI<f64>, f64>>,
    {
        v2d {
            x: self.x / factor,
            y: self.y / factor,
        }
    }

    pub fn norm(&self) -> f64 {
        (self.x.value.powi(2) + self.y.value.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Option<Self>
    where
        Quantity<D, SI<f64>, f64>: Mul<f64, Output = Quantity<D, SI<f64>, f64>>,
    {
        let mag = self.norm();
        if mag == 0.0 {
            None
        } else {
            Some(self.scalar_mul(1.0 / mag))
        }
    }

    pub fn dist(&self, other: &Self) -> f64
    where
        Quantity<D, SI<f64>, f64>: Sub<Output = Quantity<D, SI<f64>, f64>>,
    {
        self.sub(other).norm()
    }
}

// Overload the + operator.
impl<D: uom::si::Dimension> Add for v2d<D>
where
    Quantity<D, SI<f64>, f64>: Add<Output = Quantity<D, SI<f64>, f64>>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Overload the - operator.
impl<D: uom::si::Dimension> Sub for v2d<D>
where
    Quantity<D, SI<f64>, f64>: Sub<Output = Quantity<D, SI<f64>, f64>>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<D: uom::si::Dimension, DimFactor: uom::si::Dimension, OutDim: uom::si::Dimension> 
    Mul<Quantity<DimFactor, SI<f64>, f64>> for v2d<D>
where
    Quantity<D, SI<f64>, f64>: Mul<Quantity<DimFactor, SI<f64>, f64>, Output = Quantity<OutDim, SI<f64>, f64>>,
{
    type Output = v2d<OutDim>;

    fn mul(self, factor: Quantity<DimFactor, SI<f64>, f64>) -> Self::Output {
        v2d {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

// Overload the / operator to allow division with different dimensions.
impl<D: uom::si::Dimension, Factor, OutDim> Div<Factor> for v2d<D>
where
    Factor: Copy,
    OutDim: uom::si::Dimension,
    Quantity<D, SI<f64>, f64>: Div<Factor, Output = Quantity<OutDim, SI<f64>, f64>>,
{
    type Output = v2d<OutDim>;

    fn div(self, factor: Factor) -> Self::Output {
        v2d {
            x: self.x / factor,
            y: self.y / factor,
        }
    }
}