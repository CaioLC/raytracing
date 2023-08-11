use glam::Vec3;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufWriter};

const IMG_WIDTH: u32 = 1600;
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

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32
}
impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f32) -> Self {
        HitRecord { point, normal, t }
    }
    pub fn from_hit(obj_center: Vec3, ray: &Ray, t: f32) -> Self {
        let point = ray.at(t);
        let normal = (point - obj_center).normalize();
        HitRecord::new(point, normal, t)
    }
}

trait Hit {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}
impl Sphere {
    fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}
impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
        if root <= t_min || root >= t_max {
            root = (-half_b + sqrtd) / a;
            if root <= t_min || root >= t_max {
                return None;
            }
        };
        let rec = HitRecord::from_hit(self.center, ray, root);
        Some(rec)
    }
}

pub fn ray_color(ray: &Ray) -> Vec3 {
    let sphere = Sphere::new(Vec3 { x: 0.0, y: 0.0, z: -1.0 }, 0.5);
    match sphere.hit(ray, 0.0, f32::MAX) {
        Some(rec) => {
            0.5*(rec.normal + 1.0)
        }
        None => {
            let unit_direction = ray.dir.normalize();
            // dbg!(unit_direction);
            let a = 0.5 * (unit_direction.y + 1.0);
            Vec3::ONE.lerp(Vec3::new(0.5, 0.7, 1.0), a)
        }
    }

}

fn main() -> io::Result<()> {
    // Image
    let img_height: u32 = 1.0_f32.max(IMG_WIDTH as f32 / RATIO) as u32;

    // Camera
    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * (IMG_WIDTH as f32 / img_height as f32);
    let camera_center = Vec3::ZERO;

    // Define the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / IMG_WIDTH as f32;
    let pixel_delta_v = viewport_v / img_height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    // dbg!(viewport_upper_left);

    // Open device
    let f = File::create("image.ppm")?;
    let mut writer = BufWriter::new(f);
    write!(&mut writer, "P3\n{IMG_WIDTH} {img_height}\n255\n")?;

    // Render
    for j in 0..img_height {
        println!("Scanlines remaining: {:?}", (img_height - j));
        for i in 0..IMG_WIDTH {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_dir = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_dir);
            let color = ray_color(&ray);
            write_color(&mut writer, &color)?;
        }
    }
    println!("Complete.");
    Ok(())
}

fn write_color(writer: &mut BufWriter<File>, color: &Vec3) -> io::Result<()> {
    let n_color = *color * 255.999;
    write!(writer, "{} {} {}\n", n_color.x, n_color.y, n_color.z)?;
    Ok(())
}
