use glam::Vec3;
use rand::{thread_rng, Rng, random};

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
        let reflected = reflect(ray_in.dir, hit.local_normal);
        Ray::new(hit.point, reflected + self.fuzz * random_unit_vec())
    }
    fn attenuation(&self) -> Vec3 {
        self.albedo
    }
}

pub struct Dielectric {
    pub index_of_refraction: f32
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: HitRecord) -> Ray {
        let refraction_ratio = match hit.front_face {
            true => 1.0 / self.index_of_refraction,
            false => self.index_of_refraction
        };
        let unit_dir = ray_in.dir.normalize();
        let cos_theta = 1.0_f32.min(-unit_dir.dot(hit.local_normal));
        
        // let sin_theta = (1.0-cos_theta.powi(2)).sqrt();
        // let must_reflect = refraction_ratio * sin_theta > 1.0; # this creates something weird
        let new_dir;
        
        if reflectance(cos_theta, refraction_ratio) > random() {
            new_dir = reflect(unit_dir, hit.local_normal);
        } else {
            new_dir = refract(unit_dir, hit.local_normal, refraction_ratio);
        }
        Ray::new(hit.point, new_dir)
    }
    fn attenuation(&self) -> Vec3 {
        Vec3::ONE
    }
}

fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0*normal.dot(v)*normal
}

fn refract(unit_vec: Vec3, normal: Vec3, refraction_ratio: f32) -> Vec3 {
    let cos_theta = 1.0_f32.min(-unit_vec.dot(normal));
    let r_out_perp = refraction_ratio * (unit_vec + cos_theta*normal);
    let r_out_parallel = -(((1.0 - r_out_perp.length_squared()).abs()).sqrt()) * normal;
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Schlick`s approximation for reflectance.
    let r0 = ((1.0-ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0-r0) * (1.0 - cosine).powi(5)
}