use std::ops::*;
use super::angle::Angle;
use super::common::{Rotatable, Normalizable};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2D {
    pub x: f64, 
    pub y: f64
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Vec2D {
        Vec2D {x, y}
    }

    pub fn from_topule(coords: (f64, f64)) -> Vec2D {
        Vec2D{x: coords.0, y: coords.1}
    }

    pub fn as_tuple(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn length_sqr(&self) -> f64 {
        self.x.powi(2)  + self.y.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_sqr().sqrt()
    }

    pub fn set_length(&mut self, new_lenght: f64) {
        let cur_length = self.length();
        let mutiplier = new_lenght / cur_length;
        self.x *= mutiplier;
        self.y *= mutiplier;
    }

    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn dot(&self, other: &Vec2D) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Vec2D) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn orthogonal(&self) -> Vec2D {
        Vec2D::new(-self.y, self.x)
    }

    pub fn distance_sqr(&self, other: &Vec2D) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    pub fn distance(&self, other: &Vec2D) -> f64 {
        self.distance_sqr(other).sqrt()
    }

    pub fn angle_between(&self, other: &Vec2D) -> Angle {
        let cross = self.cross(other);
        let dot = self.dot(other);
        return Angle::from_atan2(cross, dot);
    }
}

impl Normalizable for Vec2D {
    fn normalize(&mut self) {
        let len = self.length();
        self.x /= len;
        self.y /= len;
    }

    fn normalized(&self) -> Vec2D {
        let len = self.length();
        Vec2D::new(self.x / len, self.y / len)
    }
}

impl Rotatable<f64> for Vec2D {
    type Output = Vec2D;
    fn rotate(&mut self, angle: f64) {
        let cos = angle.cos();
        let sin = angle.sin();
        let x = self.x * cos - self.y * sin;
        let y = self.x * sin + self.y * cos;
        self.x = x;
        self.y = y;
    }

    fn rotated(&self, angle: f64) -> Vec2D {
        let cos = angle.cos();
        let sin = angle.sin();
        let x = self.x * cos - self.y * sin;
        let y = self.x * sin + self.y * cos;
        Vec2D::new(x, y)
    }
}

impl Rotatable<&Angle> for Vec2D {
    type Output = Vec2D;
    fn rotate(&mut self, angle: &Angle) {
        let cos = angle.cos();
        let sin = angle.sin();
        let x = self.x * cos - self.y * sin;
        let y = self.x * sin + self.y * cos;
        self.x = x;
        self.y = y;
    }

    fn rotated(&self, angle: &Angle) -> Vec2D {
        let cos = angle.cos();
        let sin = angle.sin();
        let x = self.x * cos - self.y * sin;
        let y = self.x * sin + self.y * cos;
        Vec2D::new(x, y)
    }
}


#[cfg(test)]
mod test {
    use rand::Rng;
    use super::*;
    #[test]
    fn create_and_tup() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let x = rng.gen::<f64>();
            let y = rng.gen::<f64>();
            let v1 = Vec2D::new(x, y);
            let v2 = Vec2D::from_topule((x, y));
            assert_eq!(v1.x, x);
            assert_eq!(v1.y, y);
            assert_eq!(v1, v2);
            assert_eq!(v1.as_tuple(), (x, y));
        }
    }
}