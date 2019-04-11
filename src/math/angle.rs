use super::common::*;
use std::ops::*;

macro_rules! Op {
    ($op_trait: ident, $op_func: ident, $op: tt) => {
        impl $op_trait for Angle {
            type Output = Angle;
            fn $op_func(self, rhs: Angle) -> Angle {
                Angle{val: self.val $op rhs.val}
            }
        }
    };
}

macro_rules! OpAsn {
    ($op_trait: ident, $op_func: ident, $op: tt) => {
        impl $op_trait for Angle {
            fn $op_func(&mut self, rhs: Angle){
                self.val $op rhs.val;
            }
        }
    };
}

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd)]
pub struct Angle {
    val: f64
}

impl Angle {
    pub fn from_deg(angle_deg: f64) -> Angle {
        Angle {val: angle_deg.to_radians()}
    }

    pub fn from_rad(angle: f64) -> Angle {
        Angle {val: angle}
    }

    pub fn from_atan2(y: f64, x: f64) -> Angle {
        Angle {val: y.atan2(x)}
    }

    pub fn deg(&self) -> f64 {
        self.val.to_degrees()
    }

    pub fn rad(&self) -> f64 {
        self.val
    }

    pub fn cos(&self) -> f64 {
        self.val.cos()
    }

    pub fn sin(&self) -> f64 {
        self.val.sin()
    }
}

impl AlmostEq for Angle {
    fn is_eq(&self, rhs: &Angle, eps: f64) -> bool {
        (self.val - rhs.val).abs() < eps
    }
}

impl Normalizable for Angle {
    fn normalize(&mut self) {
        let mut new_val = self.val % TWO_PI;
        if new_val < 0.0 {
            new_val += TWO_PI;
        }
        self.val = new_val;
    }
    
    fn normalized(&self) -> Angle {
        let mut new_val = self.val % TWO_PI;
        if new_val < 0.0 {
            new_val += TWO_PI;
        }
        Angle{val: new_val}
    }
}

Op!(Add, add, +);
Op!(Sub, sub, -);
Op!(Mul, mul, *);
Op!(Div, div, /);

OpAsn!(AddAssign, add_assign, +=);
OpAsn!(SubAssign, sub_assign, -=);
OpAsn!(MulAssign, mul_assign, *=);
OpAsn!(DivAssign, div_assign, /=);

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;
    #[test]
    fn create() {
        for a in (0..360).step_by(2) {
            let from_deg = Angle::from_deg(a as f64);
            let from_rad = Angle::from_rad((a as f64).to_radians());
            assert!(from_deg.is_eq(&from_rad, EPS));
        }

        assert!(Angle::from_atan2(0.0, 1.0).normalized().is_eq(&Angle::from_deg(0.0), EPS));
        assert!(Angle::from_atan2(1.0, 0.0).normalized().is_eq(&Angle::from_deg(90.0), EPS));
        assert!(Angle::from_atan2(0.0, -1.0).normalized().is_eq(&Angle::from_deg(180.0), EPS));
        assert!(Angle::from_atan2(-1.0, 0.0).normalized().is_eq(&Angle::from_deg(270.0), EPS));
    }

    #[test]
    fn trig() {
        for a in (0..360).step_by(2) {
            let angle = Angle::from_deg(a as f64);
            assert_eq!(angle.cos(), (a as f64).to_radians().cos());
            assert_eq!(angle.sin(), (a as f64).to_radians().sin());
        }
    }

    #[test]
    fn conversions() {
        for a in (0..360).step_by(2) {
            let angle = Angle::from_deg(a as f64);
            assert!(angle.rad().is_eq(&(a as f64).to_radians(), EPS));
            assert!(angle.deg().is_eq(&(a as f64), EPS));
        }
    }

    #[test]
    fn ops() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let a = rng.gen::<f64>();
            let ang = Angle::from_rad(a);
            let b = rng.gen::<f64>();
            let ang2 = Angle::from_rad(b);
            assert!((ang + ang2).val.is_eq(&(a + b), EPS));
            assert!((ang - ang2).val.is_eq(&(a - b), EPS));
            assert!((ang * ang2).val.is_eq(&(a * b), EPS));
            assert!((ang / ang2).val.is_eq(&(a / b), EPS));
        }
    }

}