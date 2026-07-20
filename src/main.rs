mod color;
mod ray;
mod vec3;

use crate::color::{write_color, Color};
use crate::ray::Ray;
use crate::vec3::*;
use std::io;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = &(center - r.origin());
    let a = r.direction().length_squared();
    let h = dot(r.direction(), oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n: Vec3 = unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = unit_vector(r.direction().clone());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u64 = 400;

    let mut image_height = (image_width as f64 / aspect_ratio) as u64;

    if image_height < 1 {
        image_height = 1;
    }

    // Camera settings
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64) as f64;
    let camera_center = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let viewport_u = &Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = &Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = &(viewport_u / image_width as f64);
    let pixel_delta_v = &(viewport_v / image_height as f64);

    let viewport_upper_left = camera_center.clone()
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 as f64 * (pixel_delta_u + pixel_delta_v);

    // Output settings
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc.clone() + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center.clone();
            let r = Ray::new(camera_center.clone(), ray_direction);
            let pixel_color = ray_color(&r);
            // currently just ignoring error probably not great
            let _ = write_color(&mut io::stdout(), &pixel_color);
        }
    }
    eprintln!("Done!");
}
