use crate::vec3::{Vec3, Point3, Color};
use crate::ray::{Ray};
use std::sync::Arc;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord{
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord{
    pub fn new() -> HitRecord{
        return HitRecord{
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3){
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal}

    }
}


pub trait Hittable{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Clone)]
pub struct HittableList{
    pub content: Vec<Arc<dyn Hittable>>
}


impl Hittable for HittableList{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool{
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        for object in self.content.clone().into_iter(){
            if object.hit(r, t_min, closest_so_far, &mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        return hit_anything;
    }
}