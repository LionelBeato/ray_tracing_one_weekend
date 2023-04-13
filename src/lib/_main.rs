use ray::Ray;
use vec3::{Vector3, Color, Point};

mod ray;
mod vec3;

fn hit_sphere(center:&Point, radius:f64, r:&Ray) -> f64 {
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

fn ray_color(r:&Ray) -> Color {


    let point = &Point::new(0.0,0.0, -1.0);
    let t = hit_sphere(point, 0.5, r);


    if t > 0.0 {
        let unnormalized = r.at(t) - Vector3::new(0.0,0.0,-1.0);
        let N = unnormalized.normalize();
       return Color::new(N.x + 1.0, N.y + 1.0, N.z + 1.0) * 0.5
    }

    let unit_direction = r.dir.normalize();
    let t = 0.6 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * ((1.0 - t)) + Vector3::new(0.5, 0.7, 1.0) * t
}

fn write_color(pixel_color:Color) {
        println!("{} {} {}",
        255.99 * pixel_color.x,
        255.99 * pixel_color.y,
        255.99 * pixel_color.z
    ); 
}


fn main() {

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = image_width as f32 / aspect_ratio;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0,0.0,0.0);
    let horizontal = Vector3::new(viewport_width as f64, 0.0,0.0); 
    let vertical = Vector3::new(0.0,viewport_height as f64, 0.0);
    let lower_left_corner = origin - (horizontal/2.0) - (vertical/2.0) - Vector3::new(0.0,0.0,focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);


    for j in (0..image_height as i64).rev() {
        eprintln!("scanlines remaining: {}", j);
        for i in 0..image_width as i64 {

            let u = i as f32 / (image_width - 1) as f32; 
            let v = j as f32 / (image_height - 1.0) as f32; 

            let dir = lower_left_corner + (horizontal * u as f64) + (vertical * v as f64) - origin;
            let r = Ray::new(origin, dir);

            let pixel_color = ray_color(&r);
            write_color(pixel_color);
        }
    }
    eprintln!("Done");
}