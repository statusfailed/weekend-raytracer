extern crate nalgebra as na;
use na::{Vector3};

mod image;
mod ray;

use ray::{Point, Ray};

fn write_test_image() {
    let img = image::test_image();
    image::write_ascii(img);
}

fn main() {
    write_test_image()
}
