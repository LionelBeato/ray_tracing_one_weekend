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
    world.add(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5).into());
    world.add(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0).into());

    // camera
    let cam = Camera::new();

    // let viewport_height = 2.0;
    // let viewport_width = aspect_ratio * viewport_height;
    // let focal_length = 1.0;

    // let origin = Vector3::new(0.0,0.0,0.0);
    // let horizontal = Vector3::new(viewport_width as f64, 0.0,0.0); 
    // let vertical = Vector3::new(0.0,viewport_height as f64, 0.0);
    // let lower_left_corner = origin - (horizontal/2.0) - (vertical/2.0) - Vector3::new(0.0,0.0,focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);


    for j in (0..image_height as i64).rev() {
        eprintln!("scanlines remaining: {}", j);
        for i in 0..image_width as i64 {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel as i64 {
                // let u = i as f32 / (image_width - 1) as f32; 
                // let v = j as f32 / (image_height - 1.0) as f32; 
                let u = (i as f64 + rand::random::<f64>()) / (image_width as f64 - 1.0); 
                let v = (j as f64 + rand::random::<f64>()) / (image_height as f64 - 1.0); 
                // let dir = lower_left_corner + (horizontal * u as f64) + (vertical * v as f64) - origin;
                // let r = Ray::new(origin, dir);
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, world.to_owned(), max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("Done");
}