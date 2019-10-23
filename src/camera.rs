use crate::ray::{Ray, Point, V3};
use na::{Vector3};
use std::f64::consts::PI;

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

    pub fn new(lookfrom: V3, lookat: V3, vup: V3, vfov: f64, aspect: f64) -> Camera {
        let theta: f64 = vfov * PI / 180.0; // convert from degrees to radians
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w: V3 = (lookfrom - lookat).normalize();
        let u: V3 = vup.cross(&w).normalize();
        let v: V3 = w.cross(&u);

        let origin: V3 = lookfrom;

        Camera {
            lower_left_corner: origin - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: origin,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin,
            self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}
