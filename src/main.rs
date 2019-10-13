extern crate nalgebra as na;
use na::{Vector3};

mod image;
mod ray;

use ray::{Point, Ray};

fn write_test_image() {
    let img = image::test_image();
    image::write_ascii(img);
}

fn color(r: &Ray) -> Vector3<f64> {
    let unit_direction: Vector3<f64> = r.direction().normalize();
    let t: f64 = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0)
         + t * Vector3::new(0.5, 0.7, 1.0);
}

fn scene() -> image::P3 {
    let nx: usize = 400;
    let ny: usize = 200;
    let mut data = Vec::with_capacity(nx * ny);

    let lower_left_corner = Vector3::new(-2., -1., -1.);
    let horizontal = Vector3::new(4., 0., 0.);
    let vertical = Vector3::new(0., 2., 0.);
    let origin = Vector3::new(0., 0., 0.);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray::new(
                origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&r);

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
