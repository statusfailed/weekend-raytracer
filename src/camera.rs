use std::vec::{Vec};
use std::option::{Option};
use crate::ray::{Ray, Point, V3};
use na::{Vector3};

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: V3,
    vertical: V3,
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            lower_left_corner: Vector3::new(-2.0, -1.0, -1.0),
            horizontal: Vector3::new(4.0, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0, 0.0),
            origin: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin,
            self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}
