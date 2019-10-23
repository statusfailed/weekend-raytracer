use crate::ray::{Ray, Point, V3};
use na::{Vector3};
use std::f64::consts::PI;
use crate::random::{random_double};

fn random_in_unit_disk() -> V3 {
    let mut p: V3;
    loop {
        p = 2.0 * V3::new(random_double(), random_double(), 0.0) - V3::new(1.0, 1.0, 0.0);
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: V3,
    vertical: V3,
    u: V3,
    v: V3,
    w: V3,
    lens_radius: f64,
}

impl Camera {
    // TODO: too many params, bit messy?
    pub fn new(lookfrom: V3, lookat: V3, vup: V3, vfov: f64, aspect: f64,
               aperture: f64, focus_dist: f64) -> Camera {
        let theta: f64 = vfov * PI / 180.0; // convert from degrees to radians
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w: V3 = (lookfrom - lookat).normalize();
        let u: V3 = vup.cross(&w).normalize();
        let v: V3 = w.cross(&u);

        let origin: V3 = lookfrom;

        Camera {
            lower_left_corner : origin
                              - half_width * focus_dist * u
                              - half_height * focus_dist * v
                              - focus_dist * w,

            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: origin,
            u: u, v: v, w: w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: V3 = self.lens_radius * random_in_unit_disk();
        let offset: V3 = self.u * rd.x + self.v * rd.y;

        Ray::new(self.origin + offset,
            self.lower_left_corner
            + s*self.horizontal
            + t*self.vertical
            - self.origin
            - offset)
    }
}
