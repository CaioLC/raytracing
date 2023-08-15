use glam::Vec3;
use rand::{random, thread_rng, Rng};


pub struct Interval {
    pub t_min: f32,
    pub t_max: f32,
}
impl Interval {
    pub const EMPTY: Self = Self {
        t_min: f32::INFINITY,
        t_max: f32::NEG_INFINITY,
    };
    pub const UNIVERSE: Self = Self {
        t_min: f32::NEG_INFINITY,
        t_max: f32::INFINITY,
    };

    pub fn contains(&self, t: f32) -> bool {
        self.t_min <= t && t <= self.t_max
    }
    pub fn surrounds(&self, t: f32) -> bool {
        self.t_min < t && t < self.t_max
    }
    pub fn clamp(&self, t: f32) -> f32 {
        if t > self.t_max {
            return self.t_max;
        }
        if t < self.t_min {
            return self.t_min;
        }
        t
    }
}
impl Default for Interval {
    fn default() -> Self {
        Self {
            t_min: f32::INFINITY,
            t_max: f32::NEG_INFINITY,
        } // default interval is empty.
    }
}

pub fn random_unit_vec() -> Vec3 {
    return Vec3::new(random(), random(), random()).normalize();
}

pub fn random_vec_rng(min: f32, max: f32) -> Vec3 {
    let mut rng = thread_rng();
    return Vec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
    .normalize();
}