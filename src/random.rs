use na::Vector3;
use rand::{random, Open01};

use crate::ray::V3;

// here to keep things similar to tutorial
pub fn random_double() -> f64 {
    let Open01(val) = random::<Open01<f64>>();
    val
}

// rejection sample a point inside the unit sphere.
pub fn random_in_unit_sphere() -> V3 {
    let mut p;
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
