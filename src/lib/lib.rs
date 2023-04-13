use std::{f32::INFINITY, rc::Rc};

use hit::{Hittable, HittableList, HitRecord};
use ray::Ray;
use vec3::{Vector3, Color, Point};

pub mod ray;
pub mod vec3;
pub mod hit;
pub mod material;
pub mod sphere;
// pub mod camera;

pub fn hit_sphere(center:&Point, radius:f64, r:&Ray) -> f64 {
    let oc = r.orig - *center;
    let a = r.dir.length_squared();
    let half_b = oc.dot(&r.dir);
    let c = oc.length_squared() - (radius * radius);
    let discriminant =  (half_b * half_b) - (a * c);
    if discriminant < 0.0 {
        return -1.0
    } else {
        return ((-1.0 * half_b) - discriminant.sqrt()) / a
    }
}

pub fn ray_color(r:&Ray, world:HittableList) -> Color {

    let mut rec:HitRecord = HitRecord { 
        p: Vector3::new(0.0, 0.0, 0.0), 
        normal: Vector3::new(0.0, 0.0, 0.0), 
        // material_pointer: Rc<>::default(),
        t: 0.0,
        front_face: false 
    };

    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = r.dir.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + (Vector3::new(0.5, 0.7, 1.0) * t)
}

pub fn write_color(pixel_color:Color) {
        println!("{} {} {}",
        255.99 * pixel_color.x,
        255.99 * pixel_color.y,
        255.99 * pixel_color.z
    ); 
}