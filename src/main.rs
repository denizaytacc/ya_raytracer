mod vec3;
mod ray;
mod sphere;
mod hittable;
mod camera;
mod material;

use std::sync::Arc;
use std::rc::Rc;
use rand::Rng;
use crate::vec3::{Vec3, Point3, Color};
use crate::ray::{Ray};
use crate::sphere::{Sphere};
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::camera::{Camera};
use crate::material::{Lambertian, Scatter, Metal, Dielectric};
extern crate glium;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: u64 = 50;
    let mut rng = rand::thread_rng();

    // World
    let mut world: HittableList = HittableList{
        content: Vec::new(),
    };
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_left_other = Rc::new(Dielectric::new(1.5));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let sphere_left_other = Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, material_left_other);
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    world.content.push(Arc::new(sphere_ground));
    world.content.push(Arc::new(sphere_center));
    world.content.push(Arc::new(sphere_left));
    world.content.push(Arc::new(sphere_left_other));
    world.content.push(Arc::new(sphere_right));

    // Camera
    let camera = Camera::new();
    println!("P3\n{IMAGE_WIDTH}  {IMAGE_HEIGHT} \n255");
    for j in (0..IMAGE_HEIGHT).rev(){
        eprintln!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH{
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLES_PER_PIXEL{
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH-  1) as f64;
                let v = (j as f64+ rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += r.ray_color(world.clone(), MAX_DEPTH);
            }

            pixel_color.write_color(SAMPLES_PER_PIXEL as f64);
        }
    }
    eprintln!("Done");
}