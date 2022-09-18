use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign, Index, Range};
use std::fmt;
use rand::Rng;
pub use Vec3 as Point3;
pub use Vec3 as Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vec3{
    type Output = Self;
    fn add(self, other: Self) -> Self{
    Self{
        x: self.x + other.x,
        y: self.y + other.y,
        z: self.z + other.z,
    }

    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vec3{
    type Output = Self;
    fn sub(self, other: Self) -> Self{
    Self{
        x: self.x - other.x,
        y: self.y - other.y,
        z: self.z - other.z,
    }
    }
}

impl SubAssign for Vec3{
    fn sub_assign(&mut self, other: Self){
    *self = Vec3{
        x: self.x - other.x,
        y: self.y - other.y,
        z: self.z - other.z,
    };
    }
}

impl Neg for Vec3{
    type Output = Self;
    fn neg(self) -> Self{
    Self{
        x: self.x * -1.0,
        y: self.y * -1.0,
        z: self.z * -1.0,
    }

    }
}

impl Mul<f64> for Vec3{
    type Output = Self;
    fn mul(self, number: f64) -> Self{
    Self{
        x: self.x * number,
        y: self.y * number,
        z: self.z * number,
    }
    }
}


impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x, 
            y: self * other.y, 
            z: self * other.z,
        }
    }
}

impl Mul<Vec3> for Vec3{
    type Output = Self;
    fn mul(self, other: Vec3) -> Self{
    Self{
        x: self.x * other.x,
        y: self.y * other.y,
        z: self.z * other.z,
    }
    }
}

impl MulAssign<f64> for Vec3{
    fn mul_assign(&mut self, number: f64){
    *self = Vec3{
        x: self.x * number,
        y: self.y * number,
        z: self.z * number,
    };
    }
}


impl Div<f64> for Vec3{
    type Output = Self;
    fn div(self, number: f64) -> Self{
    return  self * (1.0 / number);
    }
}

impl DivAssign<f64> for Vec3{
    fn div_assign(&mut self, number: f64){
    *self = Vec3{
        x: self.mul(1.0 / number).x,
        y: self.mul(1.0 / number).y,
        z: self.mul(1.0 / number).z,
    }
    }
}

impl fmt::Display for Vec3{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}


impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, idx: usize) -> &f64 {
        if idx == 0{
            return &self.x;
        }
        else if idx == 1{
            return &self.y;
        }
        else if idx == 2{
            return &self.z;
        }
        else{
            return &-1.0;
        }

    }
}

impl Vec3{
    pub fn new(d1: f64, d2: f64, d3: f64) -> Vec3{
        return Vec3{
            x: d1,
            y: d2,
            z: d3,
        };
    }
    pub fn unit_vector(self) -> Vec3{
        return self / self.length();
    }
    pub fn dot(self, other: Vec3) -> f64{
        return  self.x * other.x
              + self.y * other.y
              + self.z * other.z;
    }
    pub fn cross(self, other: Vec3) -> Vec3{
        return Vec3{
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn length(self) -> f64{
        return self.length_squared().sqrt();
    }
    pub fn length_squared(self) -> f64{
        return (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
    }
    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min..max), 
            y: rng.gen_range(min..max), 
            z: rng.gen_range(min..max),
        }
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p: Vec3 = Vec3::random(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_unit_vector() -> Vec3{
        return Vec3::random_in_unit_sphere().unit_vector();
    }
    pub fn random_in_hemisphere(&self) -> Vec3{
        let in_unit_sphere: Vec3 = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(*self) > 0.0 { // In the same hemisphere as the normal
            return in_unit_sphere;

        } 
        else{
            return -in_unit_sphere;

        }
    }
    pub fn near_zero(self) -> bool{
        // Return true if the vector is close to zero in all dimensions.
        let s = 1.0e-8;
        return (self.x).abs() < s && (self.y) < s.abs() && ((self.z).abs() < s);
    }
    pub fn reflect(self, other: Vec3) -> Vec3{
        return self - 2.0 * self.dot(other) * other;
    }
}


impl Color{
    pub fn write_color(self, samples_per_pixel: f64){
        let scale = 1.0 / samples_per_pixel;
        let r: f64 = (self.x * scale).sqrt();
        let g: f64 = (self.y * scale).sqrt();
        let b: f64 = (self.z * scale).sqrt();
        println!("{} {} {}", (255.999 * r.clamp(0.0, 0.999)) as i32, (255.999 * g.clamp(0.0, 0.999)) as i32, (255.999 * b.clamp(0.0, 0.999)) as i32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vec3_add_sub() {
        let mut vec1_orig = Vec3{x: 5f64, y: 4f64, z: 3f64};
        let mut vec1 = Vec3{x: 5f64, y: 4f64, z: 3f64};
        let vec2 = Vec3{x: 1f64, y: 2f64, z: 4f64};
        let vec3 = Vec3{x: 6f64, y: 6f64, z: 7f64};
        assert_eq!(vec1 + vec2, vec3);
        assert_eq!(vec3-vec2, vec1);
        // AddAssign
        vec1 += vec2;
        assert_eq!(vec1, vec3);
        vec1 -= vec2;
        assert_eq!(vec1, vec1_orig);

    }
    #[test]
    fn test_vec3_neg() {
        let vec1 = Vec3{x: 5.0, y: 4.0, z: 2.0};
        let vec2 = Vec3{x: -5.0, y: -4.0, z: -2.0};
        assert_eq!(-vec1, vec2);
    }
    #[test]
    fn test_vec3_mul() {
        let vec1 = Vec3{x: 5.0, y: 4.0, z: 2.0};
        let vec2 = Vec3{x: 10.0, y: 8.0, z: 4.0};
        assert_eq!(vec1 * 2.0, vec2);
        assert_eq!(2.0 * vec1 , vec2);
        let mut vec3 = Vec3{x: 5.0, y: 4.0, z: 2.0};
        let vec4 = Vec3{x: 10.0, y: 8.0, z: 4.0};
        vec3 *= 2.0;
        assert_eq!(vec3, vec4);
    }
    #[test]
    fn test_vec3_div() {
        let vec1 = Vec3{x: 10.0, y: 8.0, z: 4.0};
        let vec2 = Vec3{x: 5.0, y: 4.0, z: 2.0};
        assert_eq!(vec1 / 2.0, vec2);
        let mut vec3 = Vec3{x: 10.0, y: 8.0, z: 4.0};
        let vec4 = Vec3{x: 5.0, y: 4.0, z: 2.0};
        vec3 /= 2.0;
        assert_eq!(vec3, vec4);
    }
    #[test]
    fn test_vec3_product() {
        let vec1 = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let vec2 = Vec3{x: 1.0, y: 5.0, z: 7.0};
        let vec3 = Vec3{x: -1.0, y: -4.0, z: 3.0};
        assert_eq!(vec1.cross(vec2), vec3);
        assert_eq!(vec1.dot(vec2), 32.0);
    }
    #[test]
    fn test_vec3_length() {
        let vec1 = Vec3{x: 3.0, y: 2.0, z: 5.0};
        assert_eq!(vec1.length(), (38f64).sqrt());
    }

}