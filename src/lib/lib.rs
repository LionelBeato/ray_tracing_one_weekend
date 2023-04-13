use ray::Ray;
use vec3::{Vector3, Color, Point};

pub mod ray;
pub mod vec3;

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

pub fn ray_color(r:&Ray) -> Color {


    let point = &Point::new(0.0,0.0, -1.0);
    let t = hit_sphere(point, 0.5, r);


    if t > 0.0 {
        let unnormalized = r.at(t) - Vector3::new(0.0,0.0,-1.0);
        let n = unnormalized.normalize();
       return Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5
    }

    let unit_direction = r.dir.normalize();
    let t = 0.6 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * ((1.0 - t)) + Vector3::new(0.5, 0.7, 1.0) * t
}

pub fn write_color(pixel_color:Color) {
        println!("{} {} {}",
        255.99 * pixel_color.x,
        255.99 * pixel_color.y,
        255.99 * pixel_color.z
    ); 
}