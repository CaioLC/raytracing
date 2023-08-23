use glam::Vec3;

use crate::{utils::random_unit_vec, camera::Ray};


pub trait Material {
    fn scatter(&self, ray_in: &Ray, normal: Vec3) -> (Vec3, Vec3);
}

pub struct Uniform {
    pub albedo: Vec3,
}
impl Material for Uniform {
    fn scatter(&self, _: &Ray, normal: Vec3) -> (Vec3, Vec3) {
        let rvec = random_unit_vec();
        if normal.dot(rvec) > 0.0 {
            return (rvec, self.albedo);
        }
        (-rvec, self.albedo)
    }
}

pub struct Lambertian {
    pub albedo: Vec3,
}
impl Material for Lambertian {
    fn scatter(&self, _: &Ray, normal: Vec3) -> (Vec3, Vec3) {
        let dir = normal + random_unit_vec();
        let near_zero = 1e-8;
        let is_near_zero = dir.x < near_zero && dir.y < near_zero && dir.z < near_zero;
        match is_near_zero {
            true => (normal, self.albedo),
            false => (dir, self.albedo)
        }
    }
}

pub struct Metal {
    pub albedo: Vec3,
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, normal: Vec3) -> (Vec3, Vec3) {
        let reflected = ray_in.dir - 2.0*normal.dot(ray_in.dir);
        (reflected, self.albedo)
    }
}
// pub static UNIFORM: Uniform = Uniform{albedo:0.1};
// pub static LAMBERTIAN: Lambertian = Lambertian{albedo:0.8};
// pub static METALIC: Metal = Metal{albedo:0.5};