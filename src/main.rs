extern crate nalgebra as na;
extern crate rand;

use na::{Vector3};

mod image;
mod ray;
mod scene;
mod camera;

use ray::{Point, V3, Ray};
use scene::{Hittable, HitRecord, Sphere};
use std::f64;
use camera::{Camera};
use rand::{random, Open01};

// here to keep things similar to tutorial
fn random_double() -> f64 {
    let Open01(val) = random::<Open01<f64>>();
    val
}

// rejection sample a point inside the unit sphere.
fn random_in_unit_sphere() -> V3 {
    let mut p: Vector3<f64> = Vector3::new(0., 0., 0.);
    // ehehehh https://www.reddit.com/r/rust/comments/1v9rgp/rust_has_dowhile_loops/
    while {
        // take a random point inside the unit box
        p = 2.0 * Vector3::new(random_double(), random_double(), random_double()) -
            Vector3::new(1., 1., 1.);
        // accept it as long as it's inside the sphere.
        p.norm_squared() >= 1.0
    } { };
    p
}

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

fn color<T: Hittable>(r: &Ray, world: &T) -> Vector3<f64> {
    let steps: usize = 0;
    let mut intensity: f64 = 1.0;
    let mut current_ray: Ray = Ray::new(r.a, r.b);
    let mut ret: Vector3<f64> = Vector3::new(0., 0., 0.);

    while true {
        match world.hit(&current_ray, 0.001, f64::MAX) {
            Some(hit) => {
                let target: V3 = hit.p + hit.normal + random_in_unit_sphere();
                current_ray = Ray::new(hit.p, target - hit.p);
                intensity /= 2.0;
                // return 0.5 * color(&Ray::new(hit.p, target - hit.p), world);
            },
            None => {
                ret = intensity * background(r);
                break;
            },
        }
    };

    return ret;
}

fn scene() -> image::P3 {
    let nx: usize = 400;
    let ny: usize = 200;
    let ns: usize = 100;
    let mut data = Vec::with_capacity(nx * ny);

    let lower_left_corner = Vector3::new(-2., -1., -1.);
    let horizontal = Vector3::new(4., 0., 0.);
    let vertical = Vector3::new(0., 2., 0.);
    let origin = Vector3::new(0., 0., 0.);

    //let world = Sphere { center: Vector3::new(0., 0., -1.), radius: 0.5 };
    let world = vec![
        Sphere { center: Vector3::new(0., 0., -1.), radius: 0.5 },
        Sphere { center: Vector3::new(0., -100.5, -1.), radius: 100. },
    ];

    let cam: Camera = Camera::default();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col: Vector3<f64> = Vector3::new(0., 0., 0.);
            for s in 0..ns {
                let u: f64 = ((i as f64) + random_double()) / (nx as f64);
                let v: f64 = ((j as f64) + random_double()) / (ny as f64);
                let r: Ray = cam.get_ray(u, v);
                col += color(&r, &world);
            }

            col /= (ns as f64);

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
