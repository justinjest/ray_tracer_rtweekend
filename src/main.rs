mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use crate::rtweekend::*;

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec: HitRecord = HitRecord::new();
    if world.hit(
        r,
        Interval {
            min: 0.0,
            max: INFINITY,
        },
        &mut rec,
    ) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
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

    // World settings

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

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
            let pixel_color = ray_color(&r, &world);
            // currently just ignoring error probably not great
            let _ = write_color(&mut io::stdout(), &pixel_color);
        }
    }
    eprintln!("Done!");
}
