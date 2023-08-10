use std::io::{self, BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::File;
use glam::Vec3;

const IMG_WIDTH: u32 = 1600;
const RATIO: f32 = 16.0 / 9.0;
const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / RATIO) as u32;

const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * (IMG_WIDTH as f32 / IMG_HEIGHT as f32);

pub struct Ray {
    orig: Vec3,
    dir: Vec3
}
impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + t*self.dir
    }
}

fn main() -> io::Result<()> {
    let f = File::create("image2.ppm")?;
    let mut writer = BufWriter::new(f);
    write!(&mut writer, "P3\n{IMG_WIDTH} {IMG_HEIGHT}\n255\n")?; 
    let mut color = Vec3::ZERO;
    // Render
    for j in 0..IMG_HEIGHT {
        println!("Scanlines remaining: {:?}", (IMG_HEIGHT - j));
        for i in 0..IMG_WIDTH {
            color.x = i as f32 / IMG_WIDTH as f32;
            color.y = j as f32 / IMG_HEIGHT as f32;
            color.z = 0.0;
            write_color(&mut writer, &color)?;
        }
    };
    println!("Complete.");
    Ok(())
}

fn write_color(writer: &mut BufWriter<File>, color: &Vec3) -> io::Result<()> {
    let n_color = *color * 255.999;
    write!(writer, "{} {} {}\n", n_color.x, n_color.y, n_color.z)?;
    Ok(())
}