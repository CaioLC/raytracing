use camera::Camera;
use glam::Vec3;
use rand::{random, thread_rng, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufWriter};
use std::rc::Rc;

mod camera;
mod obj;
mod utils;
use obj::{Sphere, HitCollection, Material};

const IMG_WIDTH: u32 = 400;
const RATIO: f32 = 16.0 / 9.0;

#[derive(Clone, Copy)]
struct TestMe(f32);
impl Material for TestMe{
    fn reflect(&self, normal: Vec3) -> Vec3 {
        Vec3::ZERO
    }
}

// Declare material as a static variable
static MATERIAL: TestMe = TestMe(3.2);

fn main() -> io::Result<()> {
    {
    // World
    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &MATERIAL);
    let sphere2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &MATERIAL);
    let mut world = HitCollection(Vec::new());
    world
        .0
        .push(Box::new(sphere1));
    world
        .0
        .push(Box::new(sphere2));

    let camera = Camera::new(Vec3::ZERO, RATIO, IMG_WIDTH, 100);
    camera.render(world)?;
}
    Ok(())
}

