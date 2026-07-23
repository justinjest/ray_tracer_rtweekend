use crate::rtweekend::*;
use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn tight_expansion(a: &Interval, b: &Interval) -> Interval {
        let min = f64::min(a.min, b.min);
        let max = f64::max(a.max, b.max);
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn empty() -> Interval {
        Interval {
            min: INFINITY,
            max: -INFINITY,
        }
    }

    pub fn universe() -> Interval {
        Interval {
            min: -INFINITY,
            max: INFINITY,
        }
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x > self.max {
            return self.max;
        }
        if x < self.min {
            return self.min;
        }
        x
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        let min = self.min - padding;
        let max = self.max + padding;
        Interval { min, max }
    }
}

impl Add<f64> for Interval {
    type Output = Interval;
    fn add(self, rhs: f64) -> Self::Output {
        Interval {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;
    fn add(self, rhs: Interval) -> Self::Output {
        rhs + self
    }
}
