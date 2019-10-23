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

fn random_material() -> Material {
    let choose_mat: f64 = random_double();
    if choose_mat < 0.8 {
        return materials::lambertian(V3::new(
                random_double()*random_double(),
                random_double()*random_double(),
                random_double()*random_double()));
    } else if choose_mat < 0.95 {
        return materials::metal(V3::new(
                0.5 * (1.0 + random_double()),
                0.5 * (1.0 + random_double()),
                0.5 * (1.0 + random_double())
                ), 0.5 * random_double()); // reflectance
    } else {
        return materials::dielectric(1.5);
    }
}

fn random_scene<'a>() -> Vec<Sphere> {
    let n: usize = 21;

    let floor = materials::lambertian(V3::new(0.5, 0.5, 0.5));
    let glass = materials::dielectric(1.5);
    let matte = materials::lambertian(V3::new(0.4, 0.2, 0.1));
    let metal = materials::metal(V3::new(0.7, 0.6, 0.5), 0.0);

    let mut spheres: Vec<Sphere> = Vec::with_capacity(n*n + 4);

    // 3 additional 'main' spheres
    spheres.push(Sphere { center: V3::new(0., -1000., 0.), radius: 1000., material: floor });
    spheres.push(Sphere { center: V3::new(0., 1., 0.), radius: 1.0, material: glass });
    spheres.push(Sphere { center: V3::new(-4., 1., 0.), radius: 1.0, material: matte });
    spheres.push(Sphere { center: V3::new(4., 1., 0.), radius: 1.0, material: metal });

    for a in -11..11 {
        for b in -11..11 {
            // TODO: missing something here that rejects some spheres!
            let center: V3 = V3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double());

            let material = random_material();
            spheres.push(Sphere { center: center, radius: 0.2, material: material });
        }
    };

    spheres
}

fn scene() -> image::P3 {
    let nx: usize = 600;
    let ny: usize = 300;
    let ns: usize = 100;
    let max_depth: usize = 50;

    let mut data = Vec::with_capacity(nx * ny);
    let world = random_scene();

    // Camera setup
    let lookfrom = V3::new(7.0, 2.0, 2.5);
    let lookat = V3::new(0.0, 0.0, 0.0);
    let vup = V3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).magnitude();
    let aperture = 0.2;

    let cam: Camera = Camera::new(lookfrom, lookat, vup, 40.0,
                                  nx as f64 / ny as f64,
                                  aperture,
                                  dist_to_focus);

    // Render
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
        eprintln!("{}%", 100.0 * ((ny - j) as f64 / ny as f64));
    }

    return image::P3 { width: nx, height: ny, data: data }
}

fn main() {
    image::write_ascii(scene());
}
