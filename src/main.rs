mod vec3;
mod ray;
mod sphere;
mod hittable;
mod camera;
mod material;
mod world;


use rand::Rng;
use crate::vec3::{Vec3, Point3, Color};
use crate::camera::{Camera};
use crate::world::{random_scene};
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
    let world = random_scene();

    // Camera
    let lookfrom: Point3 = Point3::new(13.0, 2.0, 3.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, 0.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    //let camera = Camera::new(Point3::new(-2.0, 2.0, 1.0), Point3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), 90.0, ASPECT_RATIO); ex1
    let camera = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    println!("P3\n{IMAGE_WIDTH}  {IMAGE_HEIGHT} \n255");
    for j in (0..IMAGE_HEIGHT).rev(){
        eprintln!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH{
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL{
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