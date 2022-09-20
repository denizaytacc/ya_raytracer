use crate::vec3::{Vec3, Point3};
use crate::ray::{Ray};
use crate::hittable::{HitRecord, Hittable};
use crate::material::{Scatter};
use std::rc::Rc;


pub struct Sphere{
    center: Point3,
    radius: f64,
    material: Rc<dyn Scatter>,
}

impl Sphere{
    pub fn new(cen: Point3, rad: f64, mat: Rc<dyn Scatter>) -> Sphere{
        return Sphere{
            center: cen,
            radius: rad,
            material: mat,
        };
    }
}

impl Hittable for Sphere{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
    
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0{
            return None;
        }
        let sqrtd = discriminant.sqrt();
    
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root{
                return None;
            }
        }
        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            material: self.material.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.material = self.material.clone();
        return Some(rec); 
    }
}

