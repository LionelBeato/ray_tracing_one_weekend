use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use rand::Rng;
use rand::thread_rng; 

/// represents either a color or point in 3D space
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn random() -> Self {
        Self {x: thread_rng().gen(), y:thread_rng().gen() , z:thread_rng().gen()}
    }

    pub fn random_range(min:f32, max:f32) -> Self {
        Self {x: thread_rng().gen_range(min..max), y:thread_rng().gen_range(min..max) , z:thread_rng().gen_range(min..max) } 
    }

    pub fn random_unit_vector() -> Self {
        Vec3::unit_vector_self(Vec3::random_in_unit_sphere())
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 { continue };
            return p;
        }
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + 
        self.y * self.y + 
        self.z * self.z
    }

    pub fn unit_vector_self(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn dot(u:Vec3, v:Vec3) -> f32 {
        (u.x * v.x) +
        (u.y * v.y) + 
        (u.z * v.z)  
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x < s && self.y < s && self.z < s
    }

    // vec3 reflect(const vec3& v, const vec3& n) {
    // return v - 2*dot(v,n)*n;

    pub fn reflect(v:Vec3, n:Vec3) -> Vec3 {
        v - (2.0 * Vec3::dot(v,n) * n)
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

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
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

impl Mul<Vec3> for f32 {
    type Output = Vec3; 

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}
 
impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {

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
