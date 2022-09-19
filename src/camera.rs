use crate::vec3::{Vec3, Point3, Color};
use crate::ray::{Ray};
use crate::sphere::{Sphere};
use crate::hittable::{HitRecord, Hittable, HittableList};

pub struct Camera{
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera{
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Camera{
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;


        let focal_length: f64 = 1.0;


        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let orig = lookfrom;
        let hor = viewport_width * u;
        let ver = viewport_height * v;

        return Camera{
            origin: lookfrom,
            horizontal: viewport_width * u,
            vertical: viewport_height * v,
            lower_left_corner: orig - hor / 2.0 - ver / 2.0 - w,
        };

    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray{
        return Ray::new(self.origin, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin);
    }
}