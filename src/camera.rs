use crate::vec3::{Vec3, Point3, Color};
use crate::ray::{Ray};
use crate::sphere::{Sphere};

pub struct Camera{
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera{
    pub fn new(lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;


        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let ho = focus_dist * viewport_width * u;
        let ve = focus_dist * viewport_height * v;

        let lower_left_corner = lookfrom - ho / 2.0 - ve / 2.0 - focus_dist * w;
        return Camera{
            origin: lookfrom,
            horizontal: ho,
            vertical: ve,
            lower_left_corner: lower_left_corner,
            u: u,
            v: v,
            lens_radius: aperture / 2.0,
        };

    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray{
        let rd: Vec3 = self.lens_radius * Vec3::random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;
        return Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset);
    }
}