use crate::vec3::{Vec3, Point3, Color};
use crate::hittable::{HitRecord, Hittable, HittableList};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray{
    pub orig: Point3,
    pub dir: Vec3,
}


impl Ray{
    pub fn new(origin: Point3, direction: Vec3) -> Ray{
        return Ray{
            orig: origin,
            dir: direction,
        }
    }
    pub fn origin(self) -> Point3{
        return self.orig;
    }
    pub fn direction(self) -> Vec3{
        return self.dir;
    }
    pub fn at(self, t: f64) -> Point3{
        return self.orig + t * self.dir;
    }
    pub fn hit_sphere(self, center: Point3, radius: f64) -> f64{
        let oc: Vec3 = self.origin() - center;
        let a = self.direction().length_squared();
        let half_b = oc.dot(self.direction());
        let c = oc.length_squared() - radius*radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return -1.0;
        }
        else{
            return (-half_b - discriminant.sqrt()) / (2.0 * a);
        }
    }
    pub fn ray_color(self, world: HittableList, depth: u64) -> Color{
        if depth <= 0 {
            // If we've exceeded the ray bounce limit, no more light is gathered
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(self, 0.001, f64::INFINITY){
            if let Some((scattered, attenuation)) = rec.material.scatter(self, &rec){
                return attenuation * Ray::ray_color(scattered, world, depth-1);

            }
            return Color::new(0.0,0.0,0.0);
        }
        let unit_direction: Vec3 = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
    }
}
