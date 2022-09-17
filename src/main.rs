mod vec3;
mod ray;
mod sphere;
mod hittable;
use crate::vec3::{Vec3, Point3, Color};
use crate::ray::{Ray};
use crate::sphere::{Sphere};
use crate::hittable::{HitRecord, Hittable};
extern crate glium;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;


    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{IMAGE_WIDTH}  {IMAGE_HEIGHT} \n255");
    for j in (0..IMAGE_HEIGHT).rev(){
        eprintln!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH{
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let color =  r.ray_color();
            color.write_color();
        }
    }
    eprintln!("Done");
    let example = Vec3{
        x: 5.0, 
        y: 3.0,
        z: 2.0,
    };

}