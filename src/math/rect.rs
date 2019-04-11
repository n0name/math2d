use std::f64;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};
use super::vector::Vec2D;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Rect {
    l: f64,
    t: f64,
    r: f64,
    b: f64
}

impl Rect {
    // Creates rectangle of all zeros
    pub fn empty() -> Rect {
        Rect{l:0.0, r: 0.0, t: 0.0, b: 0.0}
    }

    // Creates infinitely (max) large rectangle
    pub fn infinite() -> Rect {
        Rect {
            l: -f64::MAX,
            r: f64::MAX,
            b: -f64::MAX,
            t: f64::MAX
        }
    }

    // Creates an invalid rectangle (left more than right and bottom more than top)
    pub fn null() -> Rect {
        Rect {
            l: f64::MAX,
            r: -f64::MAX,
            b: f64::MAX,
            t: -f64::MAX
        }
    }

    pub fn new_ltrb(l: f64, t: f64, r: f64, b: f64) -> Rect {
        Rect {l, t, r, b}
    }

    pub fn is_valid(&self) -> bool {
        self.l <= self.r && self.b <= self.t
    }

    pub fn is_finite(&self) -> bool {
        self.width() < f64::MAX && self.height() < f64::MAX
    }

    pub fn is_empty(&self) -> bool {
        self.width() == 0.0 && self.height() == 0.0
    }

    pub fn width(&self) -> f64 {
        self.r - self.l
    }

    pub fn height(&self) -> f64 {
        self.t - self.b
    }

    pub fn center(&self) -> Vec2D {
        Vec2D::new((self.r + self.l) / 2.0, (self.t + self.b) / 2.0)
    }

    pub fn top_left(&self) -> Vec2D {
        Vec2D::new(self.l, self.t)
    }

    pub fn top_right(&self) -> Vec2D {
        Vec2D::new(self.r, self.t)
    }

    pub fn bottom_left(&self) -> Vec2D {
        Vec2D::new(self.l, self.b)
    }

    pub fn bottom_right(&self) -> Vec2D {
        Vec2D::new(self.r, self.b)
    }

    pub fn area(&self) -> f64 {
        self.width() * self.height()
    }

    pub fn size(&self) -> (f64, f64) {
        (self.width(), self.height())
    }

    pub fn inside(&self, rhs: &Vec2D) -> bool {
        rhs.x >= self.l && rhs.x <= self.r &&
        rhs.y >= self.b && rhs.y <= self.t
    }

    pub fn inflate_x(&mut self, dx: f64) {
        let half_dx = dx / 2.0;
        self.l -= half_dx;
        self.r += half_dx;
    }

    pub fn inflate_y(&mut self, dy: f64) {
        let half_dy = dy / 2.0;
        self.b -= half_dy;
        self.t += half_dy;
    }

    pub fn inflate(&mut self, dx: f64, dy: f64) {
        self.inflate_x(dx);
        self.inflate_y(dy);
    }

    pub fn inflated(&self, dx: f64, dy: f64) -> Rect {
        let mut res = self.clone();
        res.inflate(dx, dy);
        return res;
    }

    pub fn translate(&mut self, translation: &Vec2D) {
        self.l += translation.x;
        self.r += translation.x;
        self.t += translation.y;
        self.b += translation.y;
    }

    pub fn translated(&self, translation: &Vec2D) -> Rect {
        Rect {
            l: self.l + translation.x,
            r: self.r + translation.x,
            t: self.t + translation.y,
            b: self.b + translation.y
        }
    }

    pub fn clides_with(&self, other: &Rect) -> bool {
        let colides_horizontally = (other.l >= self.l && other.l <= self.r) || (other.r >= self.l && other.r <= self.r);
        let colides_verticaly = (other.b >= self.b && other.b <= self.t) || (other.t >= self.b && other.t <= self.t);
        let inside_horizontally = (other.l >= self.l && other.r <= other.r) || (other.l <= self.l && other.r >= self.r);
        let inside_vertically = (other.b >= self.b && other.t <= other.t) || (other.b <= self.b && other.t >= self.t);

        (colides_horizontally && (colides_verticaly || inside_vertically)) || (colides_verticaly && (colides_horizontally || inside_horizontally))
    }
}

// Combining operator

impl BitAnd for Rect {
    type Output = Rect;
    fn bitand(self, rhs: Rect) -> Rect {
        Rect {
            l: self.l.min(rhs.l),
            t: self.t.max(rhs.t),
            r: self.r.max(rhs.r),
            b: self.b.min(rhs.b)
        }
    }
}

impl BitAndAssign for Rect {
    fn bitand_assign(&mut self, rhs: Rect) {
        self.l = self.l.min(rhs.l);
        self.t = self.t.max(rhs.t);
        self.r = self.r.max(rhs.r);
        self.b = self.b.min(rhs.b);
    }
}

impl BitAnd<Vec2D> for Rect {
    type Output = Rect;
    fn bitand(self, rhs: Vec2D) -> Rect {
        Rect {
            l: self.l.min(rhs.x),
            t: self.t.max(rhs.y),
            r: self.r.max(rhs.x),
            b: self.b.min(rhs.y)
        }
    }
}

impl BitAndAssign<Vec2D> for Rect {
    fn bitand_assign(&mut self, rhs: Vec2D) {
        self.l = self.l.min(rhs.x);
        self.t = self.t.max(rhs.y);
        self.r = self.r.max(rhs.x);
        self.b = self.b.min(rhs.y);
    }
}

// Intersection operator

impl BitOr for Rect {
    type Output = Rect;
    fn bitor(self, rhs: Rect) -> Rect {
        Rect {
            l: self.l.max(rhs.l),
            r: self.r.min(rhs.r),
            t: self.t.min(rhs.t),
            b: self.b.max(rhs.b)
        }
    }
}

impl BitOrAssign for Rect {
    fn bitor_assign(&mut self, rhs: Rect){
        self.l = self.l.max(rhs.l);
        self.r = self.r.min(rhs.r);
        self.t = self.t.min(rhs.t);
        self.b = self.b.max(rhs.b);
    }
}


#[cfg(test)]
mod test{
    use super::*;
    use rand::Rng;

    fn rand_rect() -> Rect {
        let mut rng = rand::thread_rng();
        let (h1, h2) = (rng.gen::<f64>(), rng.gen::<f64>());
        let (v1, v2) = (rng.gen::<f64>(), rng.gen::<f64>());
        Rect::new_ltrb(
            h1.min(h2),
            v1.max(v2), 
            h1.max(h2), 
            v1.min(v2))
    }

    fn rand_vec() -> Vec2D {
        let mut rng = rand::thread_rng();
        let (x, y) = (rng.gen::<f64>(), rng.gen::<f64>());
        Vec2D::new(x, y)
    }

    #[test]
    fn create() {
        let rect = Rect::new_ltrb(1.0, 2.0, 2.0, 1.0);
        assert_eq!(rect.width(), 1.0);
        assert_eq!(rect.height(), 1.0);
        assert_eq!((rect.l, rect.r), (1.0, 2.0));
        assert_eq!((rect.b, rect.t), (1.0, 2.0));
    }
}
