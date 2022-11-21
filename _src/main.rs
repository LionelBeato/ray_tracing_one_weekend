use std::rc::Rc;

use hit::{Hittable, HitRecord};
use material::Material;
use rand::{thread_rng, Rng};
use clamp::clamp;
use vec3::Vec3;

use crate::hit::HittableList;
use crate::material::{Lambertian, Metal};
use crate::sphere::Sphere;
// use crate::vec3::Vec3; 
use crate::ray::Ray;
mod vec3;
mod ray;
mod hit;
mod sphere;
mod material;

fn write_color(input: Vec3, samples_per_pixel: i32) {
    
    let mut ir = input.x;
    let mut ig = input.y;
    let mut ib = input.z;

    let scale = 1.0 / samples_per_pixel as f32;
    ir = f32::sqrt(ir * scale);  
    ig = f32::sqrt(ig * scale);
    ib = f32::sqrt(ib * scale);

    println!("{} {} {}", 
        (256.0 * clamp(ir, 0.0, 0.999)), 
        (256.0 * clamp(ig, 0.0, 0.999)), 
        (256.0 * clamp(ib, 0.0, 0.999)), 
    ); 

}

fn ray_color(r:Ray, world:HittableList, depth:f32) -> Vec3 {

    let mut rec = HitRecord { p: 
        Vec3::random(), 
        normal: Vec3::random(),
         material_pointer: Rc::new(Lambertian { albedo: Vec3::random() }), 
         t: 0.0, 
         front_face: false
         };

    if depth <= 0.0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if world.hit(&r, 0.001, f32::INFINITY, &mut rec) {
        let mut scattered: Ray = Ray { orig: Vec3::random(), dir: Vec3::random() };
        let mut attenuation: Vec3 = Vec3::random(); 

        
        if rec.material_pointer.scatter(&r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered, world, depth - 1.0);
        }

        return Vec3::new(0.0, 0.0, 0.0); 
        // let target = rec.p + rec.normal + Vec3::random_unit_vector();
        // return 0.5 * ray_color(Ray::new(rec.p, target - rec.p), world, depth-1.0);
    }

    let unit_direction:Vec3 = Vec3::unit_vector(r.dir);
    let t = 0.6 * (unit_direction.y + 1.0);
    (1.0 - t ) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.3,0.5,1.0)

}

fn main() {

    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 100; 
    let image_height = (image_width as f32 / aspect_ratio) as i64; 
    let samples_per_pixel = 100;
    let max_depth = 50.0;

    // world

    let mut world: HittableList = HittableList { objects: Vec::new() }; 
    let material_ground = Rc::new(Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0) });
    let material_center = Rc::new(Lambertian { albedo: Vec3::new(0.7, 0.3, 0.3) });
    let material_left = Rc::new(Metal { albedo: Vec3::new(0.8, 0.8, 0.8) });
    let material_right = Rc::new(Metal { albedo: Vec3::new(0.8, 0.6, 0.2) });

    world.add(Rc::new (Sphere {center: Vec3::new(0.0,-100.5,-1.0), radius: 100.0, material_pointer: material_ground}));
    world.add(Rc::new(Sphere {center: Vec3::new(0.0,0.0,0.0), radius: 0.5, material_pointer: material_center}));
    world.add(Rc::new (Sphere {center: Vec3::new(-1.0,0.0,0.0), radius: 0.5, material_pointer: material_left}));
    world.add(Rc::new (Sphere {center: Vec3::new(1.0,0.0,0.0), radius: 0.5, material_pointer: material_right}));

    
    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height; 
    let focal_length = 1.0; 

    let origin = Vec3::new(0.0,0.0,10.0); 
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0,viewport_height, 0.0);
    let llc = origin - (horizontal/2.0) - (vertical/2.0) - Vec3::new(0.0,0.0,focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height as i64).rev() {
        eprintln!("scanlines remaining: {}", j);
        for i in 0..image_width as i64 {
            let mut pixel_color = Vec3::new(0.0,0.0,0.0);
            for h in 0..samples_per_pixel {
            
                let random_float:f32 = thread_rng().gen();
                let random_float2:f32 = thread_rng().gen();

                let u:f32 = (i as f32 + random_float) / (image_width as f32 - 1.0); 
                let v = (j as f32 + random_float2) / (image_height as f32 - 1.0); 

                let dir = llc + (u* horizontal) + (v * vertical) - origin;
                let ray = Ray::new(origin, dir);

                pixel_color = pixel_color + ray_color(ray, world.clone(), max_depth);
            }

            write_color(pixel_color, samples_per_pixel); 
        }
    }
    eprintln!("Done");
}




