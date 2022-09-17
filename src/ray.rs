use crate::vec3::{Vec3, Point3, Color};


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
    pub fn ray_color(self) -> Color{
        let t = self.hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5);
        if t > 0.0{
            let N: Vec3 = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
            return 0.5 * Color::new(N.x + 1.0, N.y + 1.0, N.z + 1.0);
        }
        
        let unit_direction: Vec3 = self.direction().unit_vector();
        let t: f64 = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
    }
}