extern crate nalgebra as na;
use na::{Vector3};

mod image;
mod ray;

use ray::{Point, Ray};

fn write_test_image() {
    let img = image::test_image();
    image::write_ascii(img);
}

// does the ray r intersect the sphere centered at "center" with radius "radius"?
fn hit_sphere(center: &Vector3<f64>, radius: f64, r: &Ray) -> bool {
    let oc = Vector3::new(0., 0., 0.) - center;
    let dir = r.direction();
    let a = dir.dot(&dir);
    let b = 2.0 * oc.dot(&dir);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b*b - 4.*a*c;
    return discriminant > 0.;
}

fn color(r: &Ray) -> Vector3<f64> {
    // if the ray intersects the sphere, show red.
    if hit_sphere(&Vector3::new(0., 0., -1.), 0.5, r) {
        return Vector3::new(1., 0., 0.);
    }

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
