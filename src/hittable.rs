use crate::vec3::{Vec3, Point3, Color};
use crate::ray::{Ray};
use crate::material::{Scatter, Lambertian};
use std::sync::Arc;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord{
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Scatter>, 
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord{
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3){
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal}

    }
}


pub trait Hittable{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HittableList{
    pub content: Vec<Arc<dyn Hittable>>
}


impl Hittable for HittableList{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        let mut temp_rec = None;
        let mut closest_so_far: f64 = t_max;
        for object in self.content.clone().into_iter(){
            if let Some(rec) = object.hit(r, t_min, closest_so_far){
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
        temp_rec
    }
}