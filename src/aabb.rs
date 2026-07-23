use crate::rtweekend::*;
use std::ops::Add;

#[derive(Copy, Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: &Interval, y: &Interval, z: &Interval) -> AABB {
        let mut aabb = AABB {
            x: *x,
            y: *y,
            z: *z,
        };
        aabb.pad_to_minimus();
        aabb
    }

    pub fn empty() -> AABB {
        let empty = Interval { min: 0.0, max: 0.0 };
        AABB {
            x: empty,
            y: empty,
            z: empty,
        }
    }

    pub fn new_from_box(box0: &AABB, box1: &AABB) -> AABB {
        let x = Interval::tight_expansion(&box0.x, &box1.x);
        let y = Interval::tight_expansion(&box0.y, &box1.y);
        let z = Interval::tight_expansion(&box0.z, &box1.z);
        AABB { x, y, z }
    }

    pub fn new_from_points(a: &Point3, b: &Point3) -> AABB {
        let mut aabb = AABB {
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
        };
        aabb.pad_to_minimus();
        aabb
    }

    fn pad_to_minimus(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta)
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta)
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta)
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

    pub fn longest_axis(&self) -> u64 {
        let x = self.x.size();
        let y = self.y.size();
        let z = self.z.size();
        if x > y {
            if x > z {
                return 0;
            } else {
                return 2;
            }
        } else {
            if y > z {
                return 1;
            } else {
                return 2;
            }
        }
    }
}

impl Add<Vec3> for &AABB {
    type Output = AABB;
    fn add(self, rhs: Vec3) -> AABB {
        AABB::new(
            &(self.x + rhs.x()),
            &(self.y + rhs.y()),
            &(self.z + rhs.z()),
        )
    }
}

impl Add<&AABB> for Vec3 {
    type Output = AABB;
    fn add(self, rhs: &AABB) -> AABB {
        rhs + self
    }
}
