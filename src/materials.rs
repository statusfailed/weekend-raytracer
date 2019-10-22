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

fn reflect(incident: &V3, normal: &V3) -> V3 {
    return incident - 2.0 * incident.dot(normal) * normal;
}

pub fn metal(albedo: V3, mut fuzz: f64) -> Material {
    fuzz = fuzz.min(1.0); // cap at 1.0
    Box::new(move |ray: &Ray, hit: &HitRecord| {
        let reflected = reflect(&ray.direction().normalize(), &hit.normal);
        let scattered = Ray::new(hit.p, reflected + fuzz*random_in_unit_sphere());
        if scattered.direction().dot(&hit.normal) > 0.0 {
            Some(Scatter {
                ray: scattered,
                attenuation: albedo,
            })
        } else {
            None
        }
    })
}

///////////////////////////////
// Dielectrics

fn refract(v: V3, n: V3, ni_over_nt: f64) -> Option<V3> {
    let uv: V3 = v.normalize();
    let dt: f64 = uv.dot(&n);
    let discriminant: f64 = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

// TODO: seriously tidy this function; very un-rusty!!
pub fn dielectric(refractive_index: f64) -> Material {
    Box::new(move |ray: &Ray, hit: &HitRecord| {
        let outward_normal: V3;
        // let reflected: V3 = reflect(&ray.direction(), &hit.normal);
        let ni_over_nt: f64;

        let attenuation: V3 = V3::new(1.0, 1.0, 1.0);

        // First decide if we're inside or outside the sphere (?)
        if ray.direction().dot(&hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = refractive_index;
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / refractive_index;
        }

        match refract(ray.direction(), outward_normal, ni_over_nt) {
            Some(refracted) => Some(Scatter {
                ray: Ray::new(hit.p, refracted),
                attenuation: attenuation,
            }),

            // TODO: should return something here; see this part of the tutorial:
            // > The reader Becker has pointed out that when there is a reflection ray the function
            // > returns false so there are no reflections
            None => None,
        }
    })
}
