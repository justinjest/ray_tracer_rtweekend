use crate::rtweekend::*;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: vec![],
            bbox: AABB {
                x: Interval::new(0.0, 0.0),
                y: Interval::new(0.0, 0.0),
                z: Interval::new(0.0, 0.0),
            },
        }
    }

    pub fn new_from_list(list: Vec<Arc<dyn Hittable>>) -> HittableList {
        HittableList {
            objects: list,
            bbox: AABB {
                x: Interval::new(0.0, 0.0),
                y: Interval::new(0.0, 0.0),
                z: Interval::new(0.0, 0.0),
            },
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object.clone());
        self.bbox = AABB::new_from_box(&self.bbox, &object.bounding_box());
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        for object in &self.objects {
            if object.hit(
                r,
                Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
