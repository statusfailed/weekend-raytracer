use std::vec::{Vec};
use std::option::{Option};
use crate::ray::{Ray, Point, V3};
use na::{Vector3};

// NOTE: we're different to the tutorial here; C++ uses an abstract class, but we just use a
// function instead.
// A material is simply a way to describe what happens when a ray hits a surface:
// Either it is absorbed (None), or it is scattered and attenuated into a new ray.
pub type Material = Box<Fn(&Ray, &HitRecord) -> Option<Scatter>>;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: V3,
}

pub struct HitRecord<'a> {
    pub t: f64,             // distance along ray where we hit
    pub p: Point,           // point in 3d space where we hit- i.e., the V3 value of p(t).
    pub normal: Point,      // normal of the surface that was hit
    pub material: &'a Material // what material did we hit?
}

// TODO: make this just a type alias for a function:
// this will allow us to have heterogeneous lists of objects.
// type Hittable = Fn(r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere<'a> {
    pub center: Point,
    pub radius: f64,
    pub material: &'a Material,
}

impl Hittable for Sphere<'_> {
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
                    material: self.material,
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
                    material: self.material,
                });
            }
        }

        return None;
    }
}

impl Hittable for Vec<Sphere<'_>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut acc: Option<HitRecord> = None;

        for sphere in self {
            let x: Option<HitRecord> = sphere.hit(r, t_min, t_max);
            acc = match (x, acc) {
                (Some(hit), Some(a)) => Some(if hit.t < a.t { hit } else { a }),
                (Some(hit), None) => Some(hit),
                (None, acc1) => acc1, // wat
                // (None, None) => None,
            }
        }
        return acc;
    }
}
