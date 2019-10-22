use crate::ray::{Ray, V3};
use crate::scene::{Material, HitRecord, Scatter};
use crate::random::{random_in_unit_sphere};

// hrm. https://medium.com/@jsen/higher-order-functions-in-rust-don-t-exist-de34b7ee81de
pub fn lambertian(albedo: V3) -> Material {
    Box::new(move |_: &Ray, hit: &HitRecord| {
        let target: V3 = hit.p + hit.normal + random_in_unit_sphere();
        Some(Scatter {
            ray: Ray::new(hit.p, target - hit.p),
            attenuation: albedo,
        })
    })
}

///////////////////////////////
// Metals

pub fn reflect(incident: &V3, normal: &V3) -> V3 {
    return incident - 2.0 * incident.dot(normal) * normal;
}

pub fn metal(albedo: V3) -> Material {
    Box::new(move |ray: &Ray, hit: &HitRecord| {
        Some(Scatter {
            ray: Ray::new(hit.p, reflect(&ray.direction().normalize(), &hit.normal)),
            attenuation: albedo,
        })
    })
}
