use crate::rtweekend::*;

#[derive(Copy, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Ray {
            orig,
            dir,
            time: 0.0,
        }
    }

    pub fn new_with_time(orig: Vec3, dir: Vec3, time: f64) -> Self {
        Self { orig, dir, time }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
