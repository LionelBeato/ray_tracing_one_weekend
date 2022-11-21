use ray::Ray;
// use vec3::Vector3;
use vector3::Vector3; 


mod ray;
mod vec3;


fn ray_color(r:&Ray) -> Vector3 {
    let unit_direction = r.dir.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vector3::new(1.0, 1.0, 1.0) * ((1.0 - t)) + Vector3::new(0.5, 0.7, 1.0) * t
}

fn write_color(pixel_color:Vector3) {
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