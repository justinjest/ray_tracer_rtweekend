use ray;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    t: f64,
}

pub trait Hittable {
    fn hit(&self, r: &ray, ray_tmin: f64, ray_tmax: f64, rec: &HitRecord) -> bool;
}
