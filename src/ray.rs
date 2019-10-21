use na::{Vector3};

pub type Point = Vector3<f64>;
pub type V3 = Vector3<f64>;

pub struct Ray {
    pub a: Point,
    pub b: Point,
}

impl Ray {
    pub fn new(a: Point, b: Point) -> Ray {
        Ray {
            a: a,
            b: b,
        }
    }

    pub fn origin(&self) -> Point {
        return self.a;
    }

    pub fn direction(&self) -> Point {
        return self.b;
    }

    pub fn point_at_parameter(&self, t: f64) -> Point {
        return self.a + t * self.b;
    }
}
