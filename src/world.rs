use crate::material::{Lambertian, Metal, Dielectric};
use crate::hittable::{HittableList};
use crate::vec3::{Point3, Color};
use crate::sphere::{Sphere};
use std::sync::Arc;
use std::rc::Rc;
use rand::Rng;

pub fn random_scene() -> HittableList{
    let mut world: HittableList = HittableList{
        content: Vec::new(),
    };
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let sphere_ground = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    world.content.push(Arc::new(sphere_ground));

    for a in -11..11{
        for b in -11..11{
            let mut rng = rand::thread_rng();
            let choose_mat = rng.gen_range(-1.0..1.0);
            let center: Point3 = Point3::new(a as f64 + 0.9 * rng.gen_range(0.0..0.9), 0.2, b as f64 + 0.9 * rng.gen_range(0.0..0.9));
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9{
                if choose_mat < 0.8{
                    // diffuse
                    let albedo = Color::random(-1.0, 1.0);
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    let sphere_ground = Sphere::new(center, 0.2, sphere_material);
                    world.content.push(Arc::new(sphere_ground));

                }
                else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    let sphere_ground = Sphere::new(center, 0.2, sphere_material);
                    world.content.push(Arc::new(sphere_ground));

                }
                else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    let sphere_ground = Sphere::new(center, 0.2, sphere_material);
                    world.content.push(Arc::new(sphere_ground));

                }
            }
        }
    }

    let material_1 = Rc::new(Dielectric::new(1.5));
    let sphere_1= Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material_1);
    world.content.push(Arc::new(sphere_1));

    let material_2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let sphere_2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material_2);
    world.content.push(Arc::new(sphere_2));

    let material_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let sphere_3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material_3);
    world.content.push(Arc::new(sphere_3));


    return world;
}