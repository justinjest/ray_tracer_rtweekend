pub use crate::color::*;
pub use crate::hittable::*;
pub use crate::hittable_list::*;
pub use crate::interval::*;
pub use crate::ray::*;
pub use crate::sphere::*;
pub use crate::vec3::*;
pub use std::io;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
