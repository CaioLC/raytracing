use crate::camera::Ray;
use crate::utils::{Interval, random_unit_vec};
use glam::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(point: Vec3, normal: Vec3, t: f32, front_face: bool, material: &'a dyn Material) -> Self {
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}
pub trait Hit {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
}

pub struct HitCollection(pub Vec<Box<dyn Hit>>);
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

pub trait Material {
    fn reflect(&self, normal: Vec3) -> Vec3;
}

struct Uniform;
impl Material for Uniform {
    fn reflect(&self, normal: Vec3) -> Vec3 {
        let rvec = random_unit_vec();
        if normal.dot(rvec) > 0.0 {
            return rvec;
        }
        -rvec
    }
}


pub struct Sphere<'a> {
    center: Vec3,
    radius: f32,
    material: &'a dyn Material
}
impl<'a> Sphere<'a> {
    pub fn new(center: Vec3, radius: f32, material: &'a dyn Material) -> Self {
        Sphere { center, radius, material }
    }
    fn to_hit_record(&self, ray: &Ray, t: f32) -> HitRecord {
        let point = ray.at(t);
        let mut normal = (point - self.center).normalize(); // this is always outward normal
        let mut front_face = true;
        if normal.dot(ray.dir) > 0.0 {
            normal *= -1.0; // if normal points in the same direction of normal, then invert normal
            front_face = false;
        }
        HitRecord::new(point, normal, t, front_face, self.material)
    }
}
impl<'a> Hit for Sphere<'a> {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let half_b = ray.dir.dot(oc);
        let c = oc.dot(oc) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        
        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let root1 = (-half_b - sqrtd) / a;
        let root2 = (-half_b + sqrtd) / a;
        match (interval.contains(root1), interval.contains(root2)) {
            (true, _) => Some(self.to_hit_record(ray, root1)),
            (false, true) =>Some(self.to_hit_record(ray, root2)),
            (false, false) => None
        }
    }
}