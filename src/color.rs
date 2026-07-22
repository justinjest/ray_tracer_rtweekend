use crate::rtweekend::*;
use std::io::{self, Write};

pub type Color = Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component <= 0.0 {
        return 0.0;
    }
    linear_component.sqrt()
}

impl From<(f64, f64, f64)> for Color {
    fn from((r, g, b): (f64, f64, f64)) -> Self {
        Color::new(r, g, b)
    }
}

pub fn write_color(stream: &mut impl Write, color: &Color) -> io::Result<()> {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    // Transform into correct color space
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Map this to within 255 bytes
    let intensity = Interval::new(0.000, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as i64;
    let gbyte = (256.0 * intensity.clamp(g)) as i64;
    let bbyte = (256.0 * intensity.clamp(b)) as i64;

    writeln!(stream, "{rbyte} {gbyte} {bbyte}")?;
    Ok(())
}
