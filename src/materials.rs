use glam::Vec3;

use crate::utils::random_unit_vec;


pub trait Material {
    fn reflect(&self, normal: Vec3) -> Vec3;
}

pub struct Uniform;
impl Material for Uniform {
    fn reflect(&self, normal: Vec3) -> Vec3 {
        let rvec = random_unit_vec();
        if normal.dot(rvec) > 0.0 {
            return rvec;
        }
        -rvec
    }
}

pub struct Lambertian;
impl Material for Lambertian {
    fn reflect(&self, normal: Vec3) -> Vec3 {
        return normal + random_unit_vec();
    }
}

pub static UNIFORM: Uniform = Uniform;
pub static LAMBERTIAN: Lambertian = Lambertian;