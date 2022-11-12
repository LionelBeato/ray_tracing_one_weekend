use hit::{Hittable};
use rand::{thread_rng, Rng};
use clamp::clamp;

use crate::hit::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Vec3; 
use crate::ray::Ray;
mod vec3;
mod ray;
mod hit;
mod sphere;

/// Function that detects if a given sphere 
/// is hit by a given ray.
// fn hit_sphere(center:Vec3, radius:f64, r:Ray) -> f64 {
//     let oc = r.orig - center;
//     let a = Vec3::dot(r.dir, r.dir); 
//     let b = 2.0 * Vec3::dot(oc, r.dir); 
//     let c = Vec3::dot(oc,oc) - radius*radius; 

//     let discriminant = (b * b) - (4.0*a*c); 
//     if discriminant < 0.0 {
//         return -1.0;
//     } else {
//         return (-b - discriminant.sqrt()) / (2.0 * a); 
//     }
// }

fn write_color(input: Vec3, samples_per_pixel: i32) {
    
    let mut ir = input.x;
    let mut ig = input.y;
    let mut ib = input.z;

    let scale = 1.0 / samples_per_pixel as f64;
    ir = f64::sqrt(ir * scale);  
    ig = f64::sqrt(ig * scale);
    ib = f64::sqrt(ib * scale);

    println!("{} {} {}", 
        (256.0 * clamp(ir, 0.0, 0.999)), 
        (256.0 * clamp(ig, 0.0, 0.999)), 
        (256.0 * clamp(ib, 0.0, 0.999)), 
    ); 

}

fn ray_color(r:Ray, world:HittableList, depth:f64) -> Vec3 {

    if depth <= 0.0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match world.hit(&r, 0.001, f64::INFINITY){
        Some(rec) => {

            let target: Vec3 = rec.p + rec.normal + Vec3::random_unit_vector();

            return 0.5 * ray_color(Ray::new(rec.p, target - rec.p), world, depth - 1.0);
        }
        None => {

        }
    }

    let unit_direction:Vec3 = Vec3::unit_vector(r.dir);
    let t = 0.6 * (unit_direction.y + 1.0);
    (1.0 - t ) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.3,0.5,1.0)
}

fn main() {

    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400; 
    let image_height = (image_width as f64 / aspect_ratio) as i64; 
    let samples_per_pixel = 100;
    let max_depth = 50.0;

    // world
    let mut world: HittableList = HittableList { objects: Vec::new() };
    world.add(Sphere {center: Vec3::new(0.0,0.0,-1.0), radius: 0.5});
    world.add(Sphere {center: Vec3::new(0.0,-100.5,-1.0), radius: 0.5});
    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height; 
    let focal_length = 1.0; 

    let origin = Vec3::new(0.0,0.0,3.0); 
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0,viewport_height, 0.0);
    let llc = origin - (horizontal/2.0) - (vertical/2.0) - Vec3::new(0.0,0.0,focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height as i64).rev() {
        eprintln!("scanlines remaining: {}", j);
        for i in 0..image_width as i64 {
            let mut pixel_color = Vec3::new(0.0,0.0,0.0);
            for h in 0..samples_per_pixel {
            
                let random_float:f64 = thread_rng().gen();
                let random_float2:f64 = thread_rng().gen();

                let u:f64 = (i as f64 + random_float) / (image_width as f64 - 1.0); 
                let v = (j as f64 + random_float2) / (image_height as f64 - 1.0); 

                let dir = llc + (u* horizontal) + (v * vertical) - origin;
                let ray = Ray::new(origin, dir);

                let mut world: HittableList = HittableList { objects: Vec::new() }; 
                world.add(Sphere {center: Vec3::new(0.0,0.0,0.0), radius: 0.5});
                world.add(Sphere {center: Vec3::new(0.0,-100.5,-1.0), radius: 100.0});

                pixel_color = pixel_color + ray_color(ray, world, max_depth);
                // let color = ray_color(ray, world, max_depth);
        }
            write_color(pixel_color, samples_per_pixel); 

        }
    }
    eprintln!("Done");
}




