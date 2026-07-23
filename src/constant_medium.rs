use crate::material::Isotropic;
use crate::rtweekend::*;

pub struct ConstantMedium {
    boundry: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundry: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> Self {
        let neg_inv_density = -1.0 / density;
        ConstantMedium {
            neg_inv_density,
            phase_function: Arc::new(Isotropic::new(tex)),
            boundry,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        if !self.boundry.hit(r, Interval::universe(), &mut rec1) {
            return false;
        }
        if !self
            .boundry
            .hit(r, Interval::new(rec1.t + 0.0001, INFINITY), &mut rec2)
        {
            return false;
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_within_boundry = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_within_boundry {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);
        rec.normal = Vec3::new(1.0, 0.0, 0.0); // no reason
        rec.front_face = true; // no reason
        rec.mat = self.phase_function.clone();

        true
    }
    fn bounding_box(&self) -> AABB {
        self.boundry.bounding_box()
    }
}
