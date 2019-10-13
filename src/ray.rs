use na::{Vector3};

pub type Point = Vector3<f64>;

pub struct Ray {
    a: Point,
    b: Point,
}

impl Ray {
    pub fn new(a: Point, b: Point) -> Ray {
        Ray {
            a: a,
            b: b,
        }
    }

    fn origin(&self) -> Point {
        return self.a;
    }

    fn point_at_parameter(&self, t: f64) -> Point {
        return self.a + t * self.b;
    }
}
