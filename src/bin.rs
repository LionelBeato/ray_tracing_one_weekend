use std::rc::Rc;

use lib::material::Lambertian;
use lib::vec3::{Point, Color};
use::lib::vec3::Vector3;
use::lib::ray::Ray;
use::lib::ray_color;
use::lib::hit::HittableList;
use::lib::write_color;
use::lib::sphere::Sphere;
use::lib::camera::Camera;
use rand::Rng;

fn main() {

    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = image_width as f32 / aspect_ratio;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // world
    let mut world = HittableList::new();

    // TODO(): figure out whats wrong with the code beloq
    // as is, surfaces are pure white
    let material_ground = Rc::new(Lambertian{ albedo: Vector3::new(0.8, 0.8, 0.8) });
    let material_center = Rc::new(Lambertian{ albedo: Vector3::new(0.7, 0.3, 0.3) });

    world.add(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, material_ground).into());
    world.add(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, material_center).into());

    // camera
    let cam = Camera::new();

    println!("P3\n{} {}\n255", image_width, image_height);


    for j in (0..image_height as i64).rev() {
        eprintln!("scanlines remaining: {}", j);
        for i in 0..image_width as i64 {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel as i64 {
                let u = (i as f64 + rand::random::<f64>()) / (image_width as f64 - 1.0); 
                let v = (j as f64 + rand::random::<f64>()) / (image_height as f64 - 1.0); 
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, world.to_owned(), max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("Done");
}