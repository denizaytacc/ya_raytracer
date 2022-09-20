use crate::vec3::{Vec3, Color};
use crate::ray::{Ray};
use crate::hittable::{HitRecord};
use rand::Rng;


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

#[derive(Clone)]
pub struct Dielectric {
    ir: f64
}


impl Metal{
    pub fn new(a: Color, f: f64) -> Metal{
        return Metal{
            albedo: a,
            fuzz: f,
        };
    }
}


impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction
        }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let mut r0 = ((1.0 - ref_idx) / (1.0 + ref_idx));
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
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

impl Scatter for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio: f64;
        if rec.front_face {
            refraction_ratio = 1.0 / self.ir
        } else {
            refraction_ratio = self.ir;
        }

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = ((-1.0) * unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = rand::thread_rng();
        let random_f64 = rng.gen::<f64>();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction;
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_f64 {
            direction = unit_direction.reflect(rec.normal);
        } else {
            direction = unit_direction.refract(rec.normal, refraction_ratio);
        };

        let scattered = Ray::new(rec.p, direction);
        return Some((scattered, Color::new(1.0, 1.0, 1.0)));
    }
}