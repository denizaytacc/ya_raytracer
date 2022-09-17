use crate::vec3::{Vec3, Point3, Color};
use crate::ray::{Ray};
use crate::hittable::{HitRecord, Hittable};

pub struct Sphere{
    center: Point3,
    radius: f64,
}

impl Sphere{
    pub fn new(cen: Point3, rad: f64) -> Sphere{
        return Sphere{
            center: cen,
            radius: rad,
        };
    }
}

impl Hittable for Sphere{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool{
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
    
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0{
            return false;
        }
        let sqrtd = discriminant.sqrt();
    
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root{
                return false;
            }
        }
    
        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return true;
    }
}

