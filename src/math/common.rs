use std::f64::consts::PI;

pub const TWO_PI: f64 = PI * 2.0;
pub const PI_2: f64 = PI / 2.0;
pub const EPS: f64 = 1E-8;

pub trait AlmostEq<RHS = Self> {
    fn is_eq(&self, rhs: &RHS, eps: f64) -> bool;
}

impl AlmostEq for f64 {
    fn is_eq(&self, rhs: &f64, eps: f64) -> bool
    {
        let res = self - rhs;
        res.abs() < eps
    }
}

pub trait Rotatable<AngleType> {
    type Output;
    fn rotate(&mut self, angle: AngleType);
    fn rotated(&self, angle: AngleType) -> Self::Output;
}


pub trait Normalizable {
    fn normalize(&mut self);
    fn normalized(&self) -> Self;
}