mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtw_image;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;

use crate::rtweekend::*;

fn bouncing_spheres() {
    let mut world = HittableList::new();

    let checker = Lambertian::new(Checker::new(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(checker),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.7 {
                    // diffuse
                    let color = Color::random() * Color::random();
                    let mat = Arc::new(Lambertian::new(color));
                    let center2 = center + Vec3::new(0.0, random_between(0.0, 0.3), 0.0);
                    world.add(Arc::new(Sphere::new_moving(center, center2, 0.2, mat)));
                } else if choose_mat < 0.85 {
                    //metal
                    let albedo = Color::new(
                        random_between(0.5, 1.0),
                        random_between(0.5, 1.0),
                        random_between(0.5, 1.0),
                    );
                    let mat = Arc::new(Metal::new(albedo, random_between(0.0, 0.5)));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                } else {
                    // glass
                    let mat = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                }
            }
        }
    }

    let material_01 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_01,
    )));

    let material_02 = Arc::new(Dielectric::new(1.50));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_02,
    )));

    let material_03 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_03,
    )));
    world = HittableList::new_from_list(vec![Arc::new(BvhNode::new(world))]);

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}

fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker = Arc::new(Lambertian::new(Checker::new(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        checker.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        checker.clone(),
    )));

    world = HittableList::new_from_list(vec![Arc::new(BvhNode::new(world))]);

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 10;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.render(&world);
}

fn earth() {
    let earth_texture = ImageTexture::new("earthmap.jpg");
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));
    let mut world = HittableList::new();
    world.add(globe);
    world = HittableList::new_from_list(vec![Arc::new(BvhNode::new(world))]);

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 10;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.look_from = Point3::new(0.0, 0.0, 12.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.render(&world);
}

fn main() {
    match 1 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        _ => println!("No case"),
    }
}
