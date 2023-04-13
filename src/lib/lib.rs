use std::{f32::INFINITY, rc::Rc, default, ptr::NonNull};
use clamp::clamp;
use hit::{Hittable, HittableList, HitRecord};
use material::{Material, Lambertian};
use ray::Ray;
use vec3::{Vector3, Color, Point};

pub mod ray;
pub mod vec3;
pub mod hit;
pub mod material;
pub mod sphere;
pub mod camera;

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

pub fn ray_color(r:&Ray, world:HittableList, depth:i64) -> Color {

    let mut rec:HitRecord = HitRecord { 
        p: Vector3::new(0.0, 0.0, 0.0), 
        normal: Vector3::new(0.0, 0.0, 0.0), 
        material_pointer: Rc::new(Lambertian { albedo: Vector3::random() }), 
        t: 0.0,
        front_face: false 
    };

    if depth <= 0 { return Color::new(0.0, 0.0, 0.0); }

    if world.hit(r, 0.0001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new(Vector3::new(0.0, 0.0, 0.0),Vector3::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0,0.0,0.0);
        if rec.material_pointer.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation + ray_color(&scattered, world, depth-1)
        }
        return Color::new(0.0, 0.0, 0.0);
        // let target:Point = rec.p + rec.normal + Vector3::random_unit_vector();
        // return ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5;
    }

    let unit_direction = r.dir.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + (Vector3::new(0.5, 0.7, 1.0) * t)
}

pub fn write_color(pixel_color:Color, samples_per_pixel:i64) {

        let mut r = pixel_color.x;
        let mut g = pixel_color.y;
        let mut b = pixel_color.z;

        let scale = 1.0 / samples_per_pixel as f64;
        r = f64::sqrt(scale * r);
        g = f64::sqrt(scale * g);
        b = f64::sqrt(scale * b);
        // r *= scale; 
        // g *= scale; 
        // b *= scale; 
        
        println!("{} {} {}",
        (256.0 * clamp(r, 0.0, 10.999)),
        (256.0 * clamp(g, 0.0, 10.999)),
        (256.0 * clamp(b, 0.0, 10.999))
    ); 
}