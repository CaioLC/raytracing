use camera::Camera;
use glam::Vec3;
use std::io;

mod camera;
mod obj;
mod utils;
mod materials;
use obj::{Sphere, HitCollection};
// use materials::{LAMBERTIAN, UNIFORM};
use materials::{Lambertian, Metal};

const IMG_WIDTH: u32 = 400;
const RATIO: f32 = 16.0 / 9.0;

// Declare material as a static variable

fn main() -> io::Result<()> {
    // Materials
    let material_ground = Lambertian{albedo: Vec3 { x: 0.8, y: 0.8, z: 0.0 }};
    let material_center = Lambertian{albedo: Vec3 { x: 0.7, y: 0.3, z: 0.3 }};
    let material_left = Metal{albedo: Vec3 { x: 0.8, y: 0.8, z: 0.8 }};
    let material_right = Metal{albedo: Vec3 { x: 0.8, y: 0.6, z: 0.2 }};

    // World
    let sphere0 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &material_ground);
    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &material_center);
    let sphere2 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &material_left);
    let sphere3 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &material_right);
    let mut world = HitCollection(Vec::new());
    world
        .0
        .push(Box::new(sphere0));
    world
        .0
        .push(Box::new(sphere1));
    world
        .0
        .push(Box::new(sphere2));
    world
        .0
        .push(Box::new(sphere3));

    let camera = Camera::new(Vec3::ZERO, RATIO, IMG_WIDTH, 100);
    camera.render(world)?;
    Ok(())
}

