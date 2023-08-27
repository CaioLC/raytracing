use crate::camera::Ray;
use crate::materials::Material;
use crate::utils::Interval;
use glam::Vec3;

pub trait Hit {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
    fn to_hit_record(&self, ray: &Ray, root: f32) -> HitRecord;
}

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub point: Vec3,
    pub local_normal: Vec3, // local means always opposite to ray.
    pub t: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(point: Vec3, local_normal: Vec3, t: f32, front_face: bool, material: &'a dyn Material) -> Self {
        HitRecord {
            point,
            local_normal,
            t,
            front_face,
            material,
        }
    }
    
    /// from a ray and a normal that always points outwards, set if ray comes from
    /// the front (front_face: True | False) and also return the local normal (always
    /// in the oposite direction of the ray)
    pub fn set_face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let inside = ray.dir.dot(outward_normal) > 0.0;
        match inside {
            true => (false, -outward_normal),
            false => (true, outward_normal)
        }

    }
}

pub struct HitCollection<'a>(pub Vec<Box<dyn Hit + 'a>>);
impl<'a> HitCollection<'a> {
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



pub struct Sphere<'a> {
    center: Vec3,
    radius: f32,
    material: &'a (dyn Material + 'a)
}
impl<'a> Sphere<'a> {
    pub fn new(center: Vec3, radius: f32, material: &'a dyn Material) -> Self {
        Sphere { center, radius, material }
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
            (false, false) => None,
        }
    }

    fn to_hit_record(&self, ray: &Ray, t: f32) -> HitRecord {
        let point = ray.at(t);
        let normal = (point - self.center) / self.radius; // this is always outward normal
        let (front_face, local_normal) = HitRecord::set_face_normal(ray, normal);
        HitRecord::new(point, local_normal, t, front_face, self.material)
    }
}