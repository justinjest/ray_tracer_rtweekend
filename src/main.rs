mod color;
mod vec3;

use crate::color::write_color;
use crate::vec3::Vec3;
use std::io;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_color = Vec3 {
                x: i as f64 / (image_width as f64 - 1.0),
                y: j as f64 / (image_height as f64 - 1.0),
                z: 0.0,
            };

            // currently just ignoring error probably not great
            let _ = write_color(&mut io::stdout(), &pixel_color);
        }
    }
    eprintln!("Done!");
}
