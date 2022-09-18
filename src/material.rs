use crate::vec3::{Vec3, Point3, Color};
use crate::ray::{Ray};
use crate::sphere::{Sphere};
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::camera::{Camera};


pub trait Scatter{
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Clone)]
pub struct Lambertian{
    albedo: Color,
}
#[derive(Clone)]
pub struct Metal{
    albedo: Color,
    fuzz: f64,
}
impl Lambertian{
    pub fn new(a: Color) -> Lambertian{
        return Lambertian{
            albedo: a
        };
    }
}

impl Scatter for Lambertian{
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Ray, Color)>{
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero(){
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        return Some((scattered, attenuation));
    }
}

impl Metal{
    pub fn new(a: Color, f: f64) -> Metal{
        return Metal{
            albedo: a,
            fuzz: f,
        };
    }
}


impl Scatter for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Ray, Color)>{
        let reflected: Vec3 = (r_in.direction()).unit_vector().reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;
        if scattered.direction().dot(rec.normal) > 0.0{
            return Some((scattered, attenuation));
        }
        else{
            return None;
        }
    }
}