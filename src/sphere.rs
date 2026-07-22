use crate::{aabb::AABB, rtweekend::*};

#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
    aabb: AABB,
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);
        let center = Ray::new(static_center, Vec3::new(0.0, 0.0, 0.0));
        Sphere {
            center,
            radius: f64::max(0.0, radius),
            mat,
            aabb: AABB::new_from_points(&(static_center - rvec), &(static_center + rvec)),
        }
    }

    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Arc<dyn Material>,
    ) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);
        let center = Ray::new(center1, center2 - center1);
        let box1 = AABB::new_from_points(&(center1 - rvec), &(center1 + rvec));
        let box2 = AABB::new_from_points(&(center2 - rvec), &(center2 + rvec));
        Sphere {
            center,
            radius: f64::max(0.0, radius),
            mat,
            aabb: AABB::new_from_box(&(box1), &(box2)),
        }
    }

    pub fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = -p.z().atan2(p.x()) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - *r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        (rec.u, rec.v) = Sphere::get_sphere_uv(&outward_normal);
        rec.mat = self.mat.clone();

        true
    }

    fn bounding_box(&self) -> AABB {
        self.aabb.clone()
    }
}
