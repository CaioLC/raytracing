use std::io;
use std::io::prelude::*;
use std::{fs::File, io::BufWriter};

use glam::Vec3;
use rand::{random, thread_rng, Rng};

use crate::obj::HitCollection;
use crate::utils::{Interval, random_unit_vec};

pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}
impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + t * self.dir
    }
}


pub struct Camera {
    image_width: u32,
    image_height: u32,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    center: Vec3,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Camera {
    pub fn new(
        position: Vec3,
        aspect_ratio: f32,
        image_width: u32,
        samples_per_pixel: u32,
    ) -> Self {
        let center = position;
        // image in pixels
        let image_height: u32 = 1.0_f32.max(image_width as f32 / aspect_ratio) as u32;

        // define viewport
        let focal_length: f32 = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);

        // calculate viewport horizontal and vertical vectors
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // calculate horizontal and vertical delta pixel
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // calculate upper left pixel
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_width,
            image_height,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            center,
            samples_per_pixel,
            max_depth: 50,
        }
    }

    pub fn render(&self, world: HitCollection) -> io::Result<()> {
        // Open device
        let f = File::create("image.ppm")?;
        let mut writer = BufWriter::new(f);
        write!(
            &mut writer,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )?;

        // Render
        for j in 0..self.image_height {
            println!("Scanlines remaining: {:?}", (self.image_height - j));
            for i in 0..self.image_width {
                let mut color = Vec3::ZERO;
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    color += self.ray_color(&ray, &world, 0);
                }
                color /= self.samples_per_pixel as f32;
                Camera::write_color(&mut writer, &color)?;
            }
        }
        println!("Complete.");
        Ok(())
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.random_sample();
        let ray_dir = pixel_sample - self.center;
        let ray = Ray::new(self.center, ray_dir);
        ray
    }

    fn ray_color(&self, ray: &Ray, world: &HitCollection, max_depth: u32) -> Vec3 {
        if max_depth < self.max_depth {
            match world.hit_any(
                ray,
                &Interval {
                    t_min: 0.0001,
                    t_max: f32::INFINITY,
                },
            ) {
                Some(rec) => {
                    // let new_dir = random_on_hemisphere(rec.normal); // uniform
                    let new_dir = rec.normal + random_unit_vec(); // lambertian dist
                    let bounce = self.ray_color(&Ray::new(rec.point, new_dir), world, max_depth+1);
                    return 0.5 * bounce;
                },
                None => {
                    let unit_direction = ray.dir.normalize();
                    let a = 0.5 * (unit_direction.y + 1.0);
                    return Vec3::ONE.lerp(Vec3::new(0.5, 0.7, 1.0), a);
                }
            }
        }
        Vec3::ZERO
    }

    fn write_color(writer: &mut BufWriter<File>, color: &Vec3) -> io::Result<()> {
        let mut gamma_color = linear_to_gamma(&color);
        let intensity = Interval { t_min: 0.0, t_max: 0.9999};
        gamma_color.x = intensity.clamp(gamma_color.x);
        gamma_color.y = intensity.clamp(gamma_color.y);
        gamma_color.z = intensity.clamp(gamma_color.z);
        gamma_color *=  256.0;

        write!(writer, "{} {} {}\n", gamma_color.x, gamma_color.y, gamma_color.z)?;
        Ok(())
    }

    fn random_sample(&self) -> Vec3 {
        let x = -0.5 + random::<f32>();
        let y = -0.5 + random::<f32>();
        Vec3 {
            x: x * self.pixel_delta_u.x,
            y: y * self.pixel_delta_v.y,
            z: 0.0,
        }
    }
}

fn linear_to_gamma(n_color: &Vec3) -> Vec3 {
    Vec3 { x: n_color.x.sqrt(), y: n_color.y.sqrt(), z: n_color.z.sqrt() }
}
