use std::vec::{Vec};
use std::option::{Option};
use crate::ray::{Ray, Point};
use na::{Vector3};

pub struct HitRecord {
    pub t: f64,
    pub p: Point,
    pub normal: Point,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = Vector3::new(0., 0., 0.) - self.center;
        let dir = r.direction();
        let a = dir.dot(&dir);
        let b = oc.dot(&dir);
        let c = oc.dot(&oc) - (self.radius * self.radius);

        let discriminant = b*b - a*c;

        // TODO: tidy this repeated code
        if discriminant > 0. {
            let mut temp: f64 = (-b - discriminant.sqrt()) / a;
            let mut p = r.point_at_parameter(temp);
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                });
            }

            // other solution: + discriminant
            temp = (-b + discriminant.sqrt()) / a;
            p = r.point_at_parameter(temp);
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                });
            }
        }

        return None;
    }
}

impl Hittable for Vec<Sphere> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut acc: Option<HitRecord> = None;

        for sphere in self {
            let x: Option<HitRecord> = sphere.hit(r, t_min, t_max);
            acc = match (x, acc) {
                (Some(hit), Some(a)) => Some(if hit.t < a.t { hit } else { a }),
                (Some(hit), None) => Some(hit),
                (None, acc1) => acc1, // wat
                (None, None) => None,
            }
        }
        return acc;
    }
}
