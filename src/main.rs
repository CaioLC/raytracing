use camera::Camera;
use glam::Vec3;
use std::io;

mod camera;
mod obj;
mod utils;
mod materials;
use obj::{Sphere, HitCollection};
// use materials::{LAMBERTIAN, UNIFORM};
use materials::{Lambertian, Metal, Dielectric};

const IMG_WIDTH: u32 = 400;
const RATIO: f32 = 16.0 / 9.0;

// Declare material as a static variable

fn main() -> io::Result<()> {
    // Materials
    let material_ground = Lambertian{albedo: Vec3 { x: 0.8, y: 0.8, z: 0.0 }};
    let material_center = Lambertian{albedo: Vec3::new(0.1, 0.2, 0.5)};
    let material_left = Dielectric{index_of_refraction: 1.5};
    let material_right = Metal{albedo: Vec3 { x: 0.8, y: 0.6, z: 0.2 }, fuzz: 0.0};

    // World
    let sphere0 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &material_ground);
    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &material_center);
    let sphere2 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &material_left);
    let sphere3 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, &material_left);
    let sphere4 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &material_right);
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
    world
        .0
        .push(Box::new(sphere4));

    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        RATIO,
        800,
        20.0,
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::Y,
        50,
        10.0,
        0.7,
    );
    camera.render(world)?;
    Ok(())
}

