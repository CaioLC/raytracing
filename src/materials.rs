use glam::Vec3;

use crate::{utils::random_unit_vec, camera::Ray, obj::HitRecord};


pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: HitRecord) -> Ray;
    fn attenuation(&self) -> Vec3;
}

pub struct Uniform {
    pub albedo: Vec3,
}
impl Material for Uniform {
    fn scatter(&self, _: &Ray, hit: HitRecord) -> Ray {
        let rvec = random_unit_vec();
        let vec = match hit.local_normal.dot(rvec) > 0.0 {
            true => rvec,
            false => -rvec
        };
        Ray::new(hit.point, vec)
    }
    fn attenuation(&self) -> Vec3 {
        self.albedo
    }
}

pub struct Lambertian {
    pub albedo: Vec3,
}
impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: HitRecord) -> Ray {
        let dir = hit.local_normal + random_unit_vec();
        let near_zero = 1e-8;
        let is_near_zero = dir.x < near_zero && dir.y < near_zero && dir.z < near_zero;
        let vec = match is_near_zero {
            true => hit.local_normal,
            false => dir
        };
        Ray::new(hit.point, vec)
    }
    fn attenuation(&self) -> Vec3 {
        self.albedo
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: HitRecord) -> Ray {
        let reflected = ray_in.dir - 2.0*hit.local_normal.dot(ray_in.dir)*hit.local_normal;
        Ray::new(hit.point, reflected + self.fuzz * random_unit_vec())
    }
    fn attenuation(&self) -> Vec3 {
        self.albedo
    }
}

pub struct Dielectric {
    pub index_of_refraction: f32
}
fn refract(unit_dir: Vec3, normal: Vec3, refraction_ratio: f32) -> Vec3 {
    let cos_theta = 1.0_f32.min(-unit_dir.dot(normal));
    let r_out_perp = refraction_ratio * (unit_dir * cos_theta*normal);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
    r_out_parallel + r_out_perp
}
impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: HitRecord) -> Ray {
        let refraction_ratio = match hit.front_face {
            true => 1.0 / self.index_of_refraction,
            false => self.index_of_refraction
        };
        let unit_direction = ray_in.dir.normalize();
        let refracted = refract(unit_direction, hit.local_normal, refraction_ratio);
        Ray::new(hit.point, refracted)
    }
    fn attenuation(&self) -> Vec3 {
        Vec3::ONE
    }
}