mod vec3;
mod ray;
mod sphere;
mod hittable;
mod camera;

use std::sync::Arc;
use rand::Rng;
use crate::vec3::{Vec3, Point3, Color};
use crate::ray::{Ray};
use crate::sphere::{Sphere};
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::camera::{Camera};
extern crate glium;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;

    let mut rng = rand::thread_rng();

    // World
    let mut world: HittableList = HittableList{
        content: Vec::new(),
    };
    world.content.push(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.content.push(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

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
                pixel_color += r.ray_color(world.clone());
            }

            pixel_color.write_color(SAMPLES_PER_PIXEL as f64);
        }
    }
    eprintln!("Done");
    let example = Vec3{
        x: 5.0, 
        y: 3.0,
        z: 2.0,
    };

}