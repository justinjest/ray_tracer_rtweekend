use crate::rtweekend::*;

#[derive(Copy, Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: &Interval, y: &Interval, z: &Interval) -> AABB {
        AABB {
            x: *x,
            y: *y,
            z: *z,
        }
    }

    pub fn new_from_box(box0: &AABB, box1: &AABB) -> AABB {
        let x = Interval::tight_expansion(&box0.x, &box1.x);
        let y = Interval::tight_expansion(&box0.y, &box1.y);
        let z = Interval::tight_expansion(&box0.z, &box1.z);
        AABB { x, y, z }
    }

    pub fn new_from_points(a: &Point3, b: &Point3) -> AABB {
        AABB {
            x: Interval {
                min: f64::min(a.x(), b.x()),
                max: f64::max(a.x(), b.x()),
            },
            y: Interval {
                min: f64::min(a.y(), b.y()),
                max: f64::max(a.y(), b.y()),
            },
            z: Interval {
                min: f64::min(a.z(), b.z()),
                max: f64::max(a.z(), b.z()),
            },
        }
    }

    pub fn axis_interval(&self, n: u64) -> Interval {
        if n == 1 {
            return self.y;
        } else if n == 2 {
            return self.z;
        } else {
            return self.x;
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
        let ray_origin = r.origin();
        let ray_direction = r.direction();

        for a in 0..3 {
            let ax = self.axis_interval(a as u64);
            let adinv = 1.0 / ray_direction[a];

            let t0 = (ax.min - ray_origin[a]) * adinv;
            let t1 = (ax.max - ray_origin[a]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}
