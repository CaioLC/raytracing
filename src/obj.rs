use crate::{Hit, HitRecord, Interval, Ray};
use glam::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}
impl Hit for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let half_b = ray.dir.dot(oc);
        let c = oc.dot(oc) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !interval.surrounds(root) {
                return None;
            }
        };
        let rec = HitRecord::from_hit(self.center, ray, root);
        Some(rec)
    }
}
