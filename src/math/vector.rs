use std::ops::*;
use super::angle::Angle;
use super::common::*;

macro_rules! Op {
    ($op_trait: ident, $op_func: ident, $op: tt) => {
        impl $op_trait for Vec2D {
            type Output = Vec2D;
            fn $op_func(self, rhs: Vec2D) -> Vec2D {
                Vec2D::new(self.x $op rhs.x, self.y $op rhs.y)
            }
        }   
    };
}

macro_rules! OpAsn {
    ($op_trait: ident, $op_func: ident, $op: tt) => {
        impl $op_trait for Vec2D {
            fn $op_func(&mut self, rhs: Vec2D) {
                self.x $op rhs.x;
                self.y $op rhs.y;
            }
        }   
    };
}


macro_rules! OpNum {
    ($op_trait: ident, $op_func: ident, $op: tt) => {
        impl $op_trait<f64> for Vec2D {
            type Output = Vec2D;
            fn $op_func(self, rhs: f64) -> Vec2D {
                Vec2D::new(self.x $op rhs, self.y $op rhs)
            }
        }   
    };
}

macro_rules! OpNumAssign {
    ($op_trait: ident, $op_func: ident, $op: tt) => {
        impl $op_trait<f64> for Vec2D {
            fn $op_func(&mut self, rhs: f64) {
                self.x $op rhs;
                self.y $op rhs;
            }
        }   
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2D {
    pub x: f64, 
    pub y: f64
}

impl Vec2D {
    pub fn zero() -> Vec2D {
        Vec2D{x: 0.0, y: 0.0}
    }

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

    pub fn angle(&self) -> Angle {
        Angle::from_atan2(self.y, self.x).normalized()
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
        if len != 0.0 {
            self.x /= len;
            self.y /= len;
        }
    }

    fn normalized(&self) -> Vec2D {
        let len = self.length();
        if len != 0.0 {
            Vec2D::new(self.x / len, self.y / len)
        } else {
            return Vec2D::zero()
        }
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

impl AlmostEq for Vec2D {
    fn is_eq(&self, rhs: &Vec2D, eps: f64) -> bool
    {
        self.x.is_eq(&rhs.x, eps) && self.y.is_eq(&rhs.y, eps)
    }
}

Op!(Add, add, +);
Op!(Sub, sub, -);
Op!(Mul, mul, *);

OpAsn!(AddAssign, add_assign, +=);
OpAsn!(SubAssign, sub_assign, -=);
OpAsn!(MulAssign, mul_assign, *=);

OpNum!(Add, add, +);
OpNum!(Sub, sub, -);
OpNum!(Mul, mul, *);
OpNum!(Div, div, /);

OpNumAssign!(AddAssign, add_assign, +=);
OpNumAssign!(SubAssign, sub_assign, -=);
OpNumAssign!(MulAssign, mul_assign, *=);
OpNumAssign!(DivAssign, div_assign, /=);


#[cfg(test)]
mod test {
    use rand::Rng;
    use super::*;

    fn gen_xys(cnt: i32) -> Vec<(f64, f64)> {
        let mut rng = rand::thread_rng();
        (0..cnt).map(move |_| {
            let x = rng.gen::<f64>();
            let y = rng.gen::<f64>();
            (x, y)
        }).collect()
    }

    #[test]
    fn create_and_tup() {
        for (x, y) in gen_xys(100) {
            let v1 = Vec2D::new(x, y);
            let v2 = Vec2D::from_topule((x, y));
            assert_eq!(v1.x, x);
            assert_eq!(v1.y, y);
            assert_eq!(v1, v2);
            assert_eq!(v1.as_tuple(), (x, y));
        }
    }

    #[test]
    fn normalization() {
        let z = Vec2D::zero().normalized();
        assert!(z.length().is_eq(&0.0, EPS));

        for (x, y) in gen_xys(100) {
            let v = Vec2D::new(x, y);
            assert!(v.normalized().length().is_eq(&1.0, EPS));
        }
    }

    #[test]
    fn length() {
        let mut rng = rand::thread_rng();
        for (x, y) in gen_xys(100) {
            let mut v = Vec2D::new(x, y);
            let len_sqr = x * x + y * y;
            assert_eq!(v.length_sqr(), len_sqr);
            assert_eq!(v.length(), len_sqr.sqrt());
            let new_len = rng.gen::<f64>();
            v.set_length(new_len);
            assert!(v.length().is_eq(&new_len, EPS));
        }
    }

    #[test]
    fn angles() {
        let tests = vec!{
            (Vec2D::new(1.0, 0.0), 0.0),
            (Vec2D::new(0.0, 1.0), 90.0),
            (Vec2D::new(-1.0, 0.0), 180.0),
            (Vec2D::new(0.0, -1.0), 270.0)
        };

        for (v, a) in tests {
            let angle = Angle::from_deg(a).normalized();
            assert!(v.angle().is_eq(&angle, EPS));
        }
    }

    #[test]
    fn rotations() {
        let v = Vec2D::new(1.0, 0.0);
        let tests = vec!{
            (Vec2D::new(1.0, 0.0), 0.0),
            (Vec2D::new(0.0, 1.0), 90.0),
            (Vec2D::new(-1.0, 0.0), 180.0),
            (Vec2D::new(0.0, -1.0), 270.0)
        };

        for (tv, a) in tests {
            let rv = v.rotated(&Angle::from_deg(a));
            let mut temp = Vec2D::new(1.0, 0.0);
            temp.rotate(&Angle::from_deg(a));
            assert!(rv.is_eq(&tv, EPS));
            assert!(temp.is_eq(&tv, EPS));
        }
    }

    #[test]
    fn ops() {
        let coords1 = gen_xys(100);
        let coords2 = gen_xys(100);
        let mut rng = rand::thread_rng();
        for (c1, c2) in coords1.iter().zip(coords2) {
            let (x1, y1) = c1;
            let (x2, y2) = c2;
            let v1 = Vec2D::new(*x1, *y1);
            let v2 = Vec2D::new(x2, y2);
            let rand = rng.gen::<f64>();

            let sum = v1 + v2;
            assert_eq!(sum.as_tuple(), (x1 + x2, y1 + y2));

            let dif = v1 - v2;
            assert_eq!(dif.as_tuple(), (x1 - x2, y1 - y2));

            let mul = v1 * v2;
            assert_eq!(mul.as_tuple(), (x1 * x2, y1 * y2));

            let sum_num = v1 + rand;
            assert_eq!(sum_num.as_tuple(), (x1 + rand, y1 + rand));

            let diff_num = v1 - rand;
            assert_eq!(diff_num.as_tuple(), (x1 - rand, y1 - rand));

            let mul_num = v1 * rand;
            assert_eq!(mul_num.as_tuple(), (x1 * rand, y1 * rand));

            let div_num = v1 / rand;
            assert_eq!(div_num.as_tuple(), (x1 / rand, y1 / rand));
        }
    }
}