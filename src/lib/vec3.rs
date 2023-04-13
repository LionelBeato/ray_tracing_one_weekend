use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use rand::Rng;
use rand::thread_rng; 

pub type Color = Vector3; 
pub type Point = Vector3;
/// represents either a color or point in 3D space
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn random() -> Self {
        Self {x: thread_rng().gen(), y:thread_rng().gen() , z:thread_rng().gen()}
    }

    pub fn random_range(min:f64, max:f64) -> Self {
        Self {x: thread_rng().gen_range(min..max), y:thread_rng().gen_range(min..max) , z:thread_rng().gen_range(min..max) } 
    }

    pub fn random_unit_vector() -> Self {
        Vector3::random_in_unit_sphere().normalize()
    }

    pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let p = Vector3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 { continue };
            return p;
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) +  self.z.powi(2)).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + 
        self.y.powi(2) + 
        self.z.powi(2)
    }

    pub fn normalize(&self) -> Vector3 {
        self.clone() / self.magnitude()
    }

    pub fn dot(self, v:&Vector3) -> f64 {
        (self.x * v.x) +
        (self.y * v.y) + 
        (self.z * v.z)  
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x < s && self.y < s && self.z < s
    }

    // vec3 reflect(const vec3& v, const vec3& n) {
    // return v - 2*dot(v,n)*n;

    pub fn reflect(v:Vector3, n:Vector3) -> Vector3 {
        v - (n * Vector3::dot(v,&n) * 2.0)
    }

}


impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
 
impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }

}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

}

impl Mul<Vector3> for Vector3 {
    type Output = Self;

    fn mul(self, other: Vector3) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::vec3::Vector3;

    #[test]
    fn length_test() {
        let vect = Vector3::new(5.0,3.0,1.0);
        let result = vect.magnitude();
        assert_eq!(result, 5.916079783099616);
    }

    #[test]
    fn division_test() {
        let vect = Vector3::new(5.0,3.0,1.0);
        let expected = Vector3 { x: 0.8451542547285166, y: 0.50709255283711, z: 0.1690308509457033 };
        let result = vect.normalize();
        assert_eq!(result, expected);
    }

    #[test]
    fn sub_test() {
        let vect = Vector3::new(5.0,3.0,1.0);
        let expected = Vector3::new(1.0, 1.0, 1.0); 
        let result = vect - Vector3::new(4.0,2.0,0.0);
        assert_eq!(result, expected);
    }

}