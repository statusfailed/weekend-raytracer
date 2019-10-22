extern crate nalgebra as na;
extern crate rand;

use na::{Vector3};

mod random;
mod image;

mod ray;
mod scene;
mod camera;
mod materials;

use ray::{V3, Ray};
use scene::{Hittable, Sphere, Material, Scatter};
use std::f64;
use camera::{Camera};
use random::{random_double};
use materials::{lambertian, metal, dielectric};

fn write_test_image() {
    let img = image::test_image();
    image::write_ascii(img);
}

// color of the background for a given ray.
fn background(r: &Ray) -> Vector3<f64> {
    let unit_direction: Vector3<f64> = r.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn color<T: Hittable>(r: &Ray, world: &T, mut depth: usize) -> Vector3<f64> {
    let mut attenuation_acc: V3 = V3::new(1.0, 1.0, 1.0);
    let mut current_ray: Ray = Ray::new(r.a, r.b);

    // max up to 50 scatters per ray
    loop {
        if depth <= 0 { break };
        depth -= 1;

        match world.hit(&current_ray, 0.001, f64::MAX) {
            Some(hit) => {
                match (hit.material)(&current_ray, &hit) {
                    // absorbed: no light, returrn zero color.
                    None => return V3::new(0., 0., 0.),

                    // scattered: multiply attenuation accumulator and continue.
                    Some(Scatter { ray, attenuation }) => {
                        // multiply-accumulate each component
                        attenuation_acc.component_mul_assign(&attenuation);
                        current_ray = ray;
                    },
                }
            },
            None => {
                // no hit -> background!
                attenuation_acc.component_mul_assign(&background(r));
                return attenuation_acc;
            },
        }
    };

    return V3::new(0., 0., 0.);
}

fn scene() -> image::P3 {
    let nx: usize = 400;
    let ny: usize = 200;
    let ns: usize = 100;
    let max_depth: usize = 50;
    let mut data = Vec::with_capacity(nx * ny);

    let lam1: Material = lambertian(V3::new(0.8, 0.3, 0.3));
    let lam2: Material = lambertian(V3::new(0.8, 0.8, 0.0));
    let met1: Material = metal(V3::new(0.8, 0.6, 0.2), 0.3);
    let die1: Material = dielectric(1.5);

    let world = vec![
        // center
        Sphere { center: Vector3::new(0., 0., -1.), radius: 0.5, material: &lam1},

        // the "floor"
        Sphere { center: Vector3::new(0., -100.5, -1.), radius: 100., material: &lam2},

        // RHS metal sphere
        Sphere { center: Vector3::new(1., 0., -1.), radius: 0.5, material: &met1},

        // hollow glass sphere
        Sphere { center: Vector3::new(-1., 0., -1.), radius: 0.5, material: &die1},
        Sphere { center: Vector3::new(-1., 0., -1.), radius: -0.45, material: &die1},
    ];

    let cam: Camera = Camera::default();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col: Vector3<f64> = Vector3::new(0., 0., 0.);
            for _ in 0..ns {
                let u: f64 = ((i as f64) + random_double()) / (nx as f64);
                let v: f64 = ((j as f64) + random_double()) / (ny as f64);
                let r: Ray = cam.get_ray(u, v);
                col += color(&r, &world, max_depth);
            }

            col /= ns as f64;

            let ir = (255.99 * col.x.sqrt()) as u8;
            let ig = (255.99 * col.y.sqrt()) as u8;
            let ib = (255.99 * col.z.sqrt()) as u8;

            data.push((ir, ig, ib));
        }
    }

    return image::P3 { width: nx, height: ny, data: data }
}

fn main() {
    image::write_ascii(scene());
}
