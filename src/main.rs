extern crate nalgebra as na;
use na::{Vector3};

mod image;
mod ray;
mod scene;

use ray::{Point, Ray};
use scene::{Hittable, HitRecord, Sphere};
use std::f64;

fn write_test_image() {
    let img = image::test_image();
    image::write_ascii(img);
}

fn color<T: Hittable>(r: &Ray, world: &T) -> Vector3<f64> {
    // if the ray intersects the sphere, show red.
    match world.hit(r, 0.0, f64::MAX) {
        Some(hit) => {
            return 0.5 * Vector3::new(
                hit.normal.x + 1., hit.normal.y + 1., hit.normal.z + 1.);
        },
        None => {
            let unit_direction: Vector3<f64> = r.direction().normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0)
                 + t * Vector3::new(0.5, 0.7, 1.0);
        },
    }


}

fn scene() -> image::P3 {
    let nx: usize = 400;
    let ny: usize = 200;
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

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray::new(
                origin, lower_left_corner + u * horizontal + v * vertical);

            let p = r.point_at_parameter(2.0);
            let col = color(&r, &world);

            let ir = (255.99 * col.x) as u8;
            let ig = (255.99 * col.y) as u8;
            let ib = (255.99 * col.z) as u8;

            data.push((ir, ig, ib));
        }
    }

    return image::P3 { width: nx, height: ny, data: data }
}

fn main() {
    image::write_ascii(scene());
}
