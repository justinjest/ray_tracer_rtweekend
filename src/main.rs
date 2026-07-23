mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod quad;
mod ray;
mod rtw_image;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;

use crate::rtweekend::*;

fn generic_camera() -> Camera {
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;
    cam
}

fn cornell_smoke() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));

    let box1 = generate_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );

    let r_box1 = Arc::new(RotateY::new(box1, 15.0));
    let r_t_box1 = Arc::new(Translate::new(r_box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(Arc::new(ConstantMedium::new(
        r_t_box1,
        0.01,
        Color::new(0.0, 0.0, 0.0).into(),
    )));

    let box2 = generate_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );

    let r_box2 = Arc::new(RotateY::new(box2, -18.0));
    let r_t_box2 = Arc::new(Translate::new(r_box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(Arc::new(ConstantMedium::new(
        r_t_box2,
        0.01,
        Color::new(1.0, 1.0, 1.0).into(),
    )));
    let mut cam = generic_camera();
    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.background = Color::new(0.0, 0.0, 0.0);
    cam.vfov = 40.0;
    cam.look_from = Point3::new(278.0, 278.0, -800.0);
    cam.look_at = Point3::new(278.0, 278.0, 0.0);
    cam.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ))); // left

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ))); // right

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ))); // bottom

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ))); // top

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ))); // back

    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));

    let box1 = generate_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );

    let r_box1 = Arc::new(RotateY::new(box1, 15.0));
    let r_t_box1 = Arc::new(Translate::new(r_box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(r_t_box1);

    let box2 = generate_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );

    let r_box2 = Arc::new(RotateY::new(box2, -18.0));
    let r_t_box2 = Arc::new(Translate::new(r_box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(r_t_box2);
    let mut cam = generic_camera();
    cam.aspect_ratio = 1.0;
    cam.image_width = 200;
    cam.samples_per_pixel = 500;
    cam.background = Color::new(0.0, 0.0, 0.0);
    cam.vfov = 40.0;
    cam.look_from = Point3::new(278.0, 278.0, -800.0);
    cam.look_at = Point3::new(278.0, 278.0, 0.0);
    cam.render(&world);
}

fn simple_lights() {
    let mut world = HittableList::new();

    let pertex = Arc::new(Lambertian::new(NoiseTexture::new(4.0)));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        pertex.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        pertex,
    )));

    let difflight = Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight,
    )));

    let mut cam = generic_camera();
    cam.background = Color::new(0.0, 0.0, 0.0);
    cam.vfov = 20.0;
    cam.look_from = Point3::new(26.0, 3.0, 6.0);
    cam.look_at = Point3::new(0.0, 2.0, 0.0);

    cam.render(&world);
}

fn quads() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let green = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let orange = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let teal = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        red,
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        green,
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        blue,
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        orange,
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        teal,
    )));

    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.vfov = 80.0;
    cam.look_from = Point3::new(0.0, 0.0, 9.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);

    cam.render(&world);
}

fn perlin_spheres() {
    let mut world = HittableList::new();

    let pertex = Arc::new(Lambertian::new(NoiseTexture::new(4.0)));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        pertex.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        pertex.clone(),
    )));

    generic_camera().render(&world);
}

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
                    let mat = Arc::new(Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                }
            }
        }
    }

    let material_01 = Arc::new(Metal::new(Color::new(0.99, 0.75, 0.79), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_01,
    )));

    let material_02 = Arc::new(Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.50));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_02,
    )));

    let material_04 = Arc::new(Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        0.9,
        material_04,
    )));

    let material_03 = Arc::new(Metal::new(NoiseTexture::new(4.0), 0.2));
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
    match 8 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_lights(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        _ => println!("No case"),
    }
}
