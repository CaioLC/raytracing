use std::{fs::File, io::BufWriter};
use std::io::prelude::*;
use std::io;

use glam::Vec3;

use crate::{HitCollection, Ray, Interval};

pub struct Camera {
    aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    center: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, aspect_ratio: f32, image_width: u32) -> Self {
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
            aspect_ratio,
            image_width,
            image_height,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            center
        }
    }

    pub fn render(&self, world: &HitCollection) -> io::Result<()> {
        // Open device
        let f = File::create("image.ppm")?;
        let mut writer = BufWriter::new(f);
        write!(&mut writer, "P3\n{} {}\n255\n", self.image_width, self.image_height)?;

        // Render
        for j in 0..self.image_height {
            println!("Scanlines remaining: {:?}", (self.image_height - j));
            for i in 0..self.image_width {
                let pixel_center =
                    self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
                let ray_dir = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_dir);
                let color = Camera::ray_color(&ray, &world);
                Camera::write_color(&mut writer, &color)?;
            }
        }
        println!("Complete.");
        Ok(())
    }

    fn ray_color(ray: &Ray, world: &HitCollection) -> Vec3 {
        match world.hit_any(
            ray,
            &Interval {
                t_min: 0.0,
                t_max: f32::INFINITY,
            },
        ) {
            Some(rec) => 0.5 * (rec.normal + 1.0),
            None => {
                let unit_direction = ray.dir.normalize();
                let a = 0.5 * (unit_direction.y + 1.0);
                Vec3::ONE.lerp(Vec3::new(0.5, 0.7, 1.0), a)
            }
        }
    }

    fn write_color(writer: &mut BufWriter<File>, color: &Vec3) -> io::Result<()> {
        let n_color = *color * 255.999;
        write!(writer, "{} {} {}\n", n_color.x, n_color.y, n_color.z)?;
        Ok(())
    }
}