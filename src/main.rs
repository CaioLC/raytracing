use camera::Camera;
use glam::Vec3;
use rand::random;
use std::io;

mod camera;
mod obj;
mod utils;
mod materials;
use obj::{Sphere, HitCollection};
// use materials::{LAMBERTIAN, UNIFORM};
use materials::{Lambertian, Metal, Dielectric, Material};

const IMG_WIDTH: u32 = 400;
const RATIO: f32 = 16.0 / 9.0;

// Declare material as a static variable
pub struct Materials<'a>(pub Vec<Box<dyn Material + 'a>>);


fn main() -> io::Result<()> {
    
    let ground_material = Lambertian {albedo: Vec3::new(0.5, 0.5, 0.5)};
    let world_sphere = Sphere::new(Vec3::new(0.0, -1000., 0.0), 1000.0, &ground_material);
    
    let mut materials = Materials(Vec::new());
    let mut positions = Vec::new();
    
    for a in -11..11 {
        for b in -11..11 {
                let choose_mat: f32 = random();
                let pos = Vec3::new(a as f32+0.9*random::<f32>(), 0.2, b as f32+0.9*random::<f32>());
                if (pos - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                        if choose_mat < 0.8 {
                            let albedo = utils::random_vec_rng(0.0, 1.0) * utils::random_vec_rng(0.0, 1.0);
                            let sphere_material = Lambertian {albedo};
                            materials.0.push(Box::new(sphere_material));
                            positions.push(pos);
                        } else if choose_mat < 0.95 {
                                let albedo = utils::random_vec_rng(0.5, 1.0);
                                let fuzz = utils::random_range(0.0, 0.5);
                                let sphere_material = Metal {albedo, fuzz};
                                materials.0.push(Box::new(sphere_material));
                                positions.push(pos);
                            } else {
                                    let sphere_material = Dielectric {index_of_refraction: 1.5};
                                    materials.0.push(Box::new(sphere_material));
                                    positions.push(pos);
                                }
            }
        }
    }
    let dielectric = Dielectric {index_of_refraction: 1.5};
    let metal = Metal {albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0};
    let lambertian = Lambertian {albedo: Vec3::new(0.4, 0.2, 0.1)};
    
    
    let mut world = HitCollection(Vec::new());
    world.0.push(Box::new(world_sphere));
    for (pos, material) in positions.iter().zip(materials.0.iter()) {
        let sphere = Sphere::new(*pos, 0.2, material.as_ref());
        world.0.push(Box::new(sphere));
    }
    
    let sphere = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, &dielectric);
    world.0.push(Box::new(sphere));
    
    let sphere = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, &lambertian);
    world.0.push(Box::new(sphere));

    let sphere = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, &metal);
    world.0.push(Box::new(sphere));



    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        RATIO,
        1200,
        20.0,
        Vec3::ZERO,
        Vec3::Y,
        500,
        10.0,
        0.6,
    );
    camera.render(&world)?;
    // drop(world);
    // drop(materials);
    Ok(())
}

