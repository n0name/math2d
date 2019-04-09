use super::common::*;

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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create() {
        for a in (0..360).step_by(2) {
            let from_deg = Angle::from_deg(a as f64);
            let from_rad = Angle::from_rad((a as f64).to_radians());
            assert!(from_deg.is_eq(&from_rad, EPS));
        }
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

}