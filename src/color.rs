use crate::vec3::Vec3;
use std::io::{self, Write};
pub fn write_color(stream: &mut impl Write, color: &Vec3) -> io::Result<()> {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    // Map this to within 255 bytes
    let rbyte = (255.999 * r) as i64;
    let gbyte = (255.999 * g) as i64;
    let bbyte = (255.999 * b) as i64;

    writeln!(stream, "{rbyte} {gbyte} {bbyte}")?;
    Ok(())
}
