use camera::Camera;
use glam::Vec3;
use rand::{random, thread_rng, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufWriter};

mod camera;
mod obj;
use obj::Sphere;

const IMG_WIDTH: u32 = 400;
const RATIO: f32 = 16.0 / 9.0;

pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}
impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + t * self.dir
    }
}

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}
impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f32, front_face: bool) -> Self {
        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
    }
    pub fn from_hit(obj_center: Vec3, ray: &Ray, t: f32) -> Self {
        let point = ray.at(t);
        let mut normal = (point - obj_center).normalize(); // this is always outward normal
        let mut front_face = true;
        if normal.dot(ray.dir) > 0.0 {
            normal *= -1.0; // if normal points in the same direction of normal, then invert normal
            front_face = false;
        }
        HitRecord::new(point, normal, t, front_face)
    }
}

pub struct Interval {
    t_min: f32,
    t_max: f32,
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

trait Hit {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
}

pub struct HitCollection(Vec<Box<dyn Hit>>);
impl HitCollection {
    pub fn hit_any(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let mut closest_so_far = interval.t_max;
        let mut hit_record = None;
        for obj in self.0.iter() {
            if let Some(hit) = obj.hit(
                ray,
                &Interval {
                    t_min: interval.t_min,
                    t_max: closest_so_far,
                },
            ) {
                hit_record = Some(hit);
                closest_so_far = hit.t;
            }
        }
        hit_record
    }
}

fn main() -> io::Result<()> {
    // World
    let mut world = HitCollection(Vec::new());
    world
        .0
        .push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world
        .0
        .push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new(Vec3::ZERO, RATIO, IMG_WIDTH, 100);
    camera.render(&world)?;
    Ok(())
}

