use camera::Camera;
use glam::Vec3;
use std::io;

mod camera;
mod obj;
mod utils;
mod materials;
use obj::{Sphere, HitCollection};
use materials::LAMBERTIAN;

const IMG_WIDTH: u32 = 400;
const RATIO: f32 = 16.0 / 9.0;

// Declare material as a static variable

fn main() -> io::Result<()> {
    // World
    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &LAMBERTIAN);
    let sphere2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &LAMBERTIAN);
    let mut world = HitCollection(Vec::new());
    world
        .0
        .push(Box::new(sphere1));
    world
        .0
        .push(Box::new(sphere2));

    let camera = Camera::new(Vec3::ZERO, RATIO, IMG_WIDTH, 100);
    camera.render(world)?;
    Ok(())
}

