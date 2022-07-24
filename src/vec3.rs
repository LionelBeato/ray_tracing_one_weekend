use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

/// represents either a color or point in 3D space
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + 
        self.y * self.y + 
        self.z * self.z
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn dot(u:Vec3, v:Vec3) -> f64 {
        (u.x * v.x) +
        (u.y * v.y) + 
        (u.z * v.z)  
    }

}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.y + other.y,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.y * other,
        }
    }
}


impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.y * other.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3; 

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}
 
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {

        (1.0 / other) * self
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            // x: other.x - self.x,
            // y: other.y - self.y,
            // z: other.z - self.z,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
        
    }
}
