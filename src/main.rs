use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

fn ray_color(r: Ray) -> Vec3 {
    // eprintln!("value from r: {:?}", r);
    let unit_direction = Vec3::unit_vector(r.dir);
    // eprintln!("value from uni_dir: {:?}", unit_direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.;
    let image_height = image_width / aspect_ratio;
    // eprintln!("{}", image_height);

    // camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // eprintln!("value of llc: {:?}", lower_left_corner);

    // render

    println!("P3\n{} {}\n255", image_width, image_height);

    for i in (0..image_height as i32).rev() {
        eprintln!("\rScanlines remaing: {} ", i);
        for j in 0..image_width as i32 {
            let u = j as f64 / image_width;
            let v = i as f64 / image_height;
            // eprintln!("value of width and height: {}, {}", u, v);

            // let pixel_color: Vec3 = Vec3::new(
            //     j as f64 / (image_width) as f64,
            //     i as f64 / (image_width) as f64,
            //     0.25,
            // );

            // eprintln!("testing dir: {:?}", lower_left_corner + horizontal * u + vertical * v - origin);
            let dir = lower_left_corner + (horizontal * u) + (vertical * v) - origin;

            let r: Ray = Ray::new(origin, dir);

            // eprintln!("value of ray: {:?}", r);

            let pixel_color: Vec3 = ray_color(r);
            write_color(pixel_color);
        }
    }
}

fn write_color(pixel_color: Vec3) -> () {
    // Write the translated [0,255] value of each color component
    // eprintln!("pixel_color: {:?}", pixel_color);
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x as f64) as i32,
        (255.999 * pixel_color.y as f64) as i32,
        (255.999 * pixel_color.z as f64) as i32
    )
}

// represents either a color or point in 3D space
#[derive(PartialEq, Clone, Copy, Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z }
    }

    fn length(self) -> f64 {
        // eprintln!("self, sqrd, sqrt: {:?}, {:?}", self, self.length_squared());
        let sqrt = f64::sqrt(self.length_squared());
        // eprintln!("sqrt: {}", sqrt);
        sqrt
    }

    fn length_squared(self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    fn unit_vector(v: Vec3) -> Vec3 {
        // eprintln!("unit vec self print: {:?} and {}", v, v.length());
        v / v.length()
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

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        // eprintln!("dividing {:?} with {}", self, other);
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }

    // fn div(self) -> Self {
    //     self * (1.0)
    // }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    fn new(origin: Vec3, dir: Vec3) -> Self {
        return Self {
            origin: origin,
            dir: dir,
        };
    }

    fn at(self, t: f64) -> Vec3 {
        return self.origin + (self.dir * t);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn div_test() {
        let expected = Vec3::new(3.0, 3.0, 3.0);
        let actual = Vec3::new(6.0, 6.0, 6.0) / 2.0;

        assert_eq!(expected, actual);
    }

    #[test]
    fn length_test() {

     // fn length_squared(self) -> f64 {
     //   self.x * self.x + self.y * self.y + self.z + self.z
    // }
        
        let v = Vec3::new(00.9244444444444446 , -1.0, -1.0);
        let v_sqrd = v.length_squared();
        let expected = (v.x * v.x) + (v.y * v.y) + (v.z * v.z);

        assert_eq!(expected, v_sqrd);
    }

    #[test]
    fn sqrt_test() {
        let v = Vec3::new(00.9244444444444446 , -1.0, -1.0);
        let v_sqrd = v.length_squared(); 
        let sqrt = f64::sqrt(v_sqrd);

        assert_eq!(0.0, sqrt); 

    }
}