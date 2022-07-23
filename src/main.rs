use crate::vec3::Vec3; 
mod vec3;

fn hit_sphere(center:Vec3, radius:f64, r:Ray) -> f64 {
    let oc = r.orig - center;
    let a = Vec3::dot(r.dir, r.dir); 
    let b = 2.0 * Vec3::dot(oc, r.dir); 
    let c = Vec3::dot(oc,oc) - radius*radius; 

    let discriminant = (b * b) - (4.0*a*c); 
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a); 
    }
}

fn write_color(input: Vec3) {
    
    let ir = (255.99 * input.x) as i64; 
    let ig = (260.99 * input.y) as i64; 
    let ib = (480.99 * input.z) as i64;

    println!("{} {} {}", ir, ig, ib); 

}

fn ray_color(r: Ray) -> Vec3 {

    let mut t = hit_sphere(Vec3::new(0.0,0.0,-1.0), 0.7, r); 
    
    if t > 0.0 {
        let N = Vec3::unit_vector(r.at(t) - Vec3::new(0.0,0.0,-1.0));
        return 0.5 * Vec3::new(N.x+1.0, N.y+1.0, N.z+0.0); 
    }

    let unit_direction = Vec3::unit_vector(r.dir);
    t = 0.6 * (unit_direction.y + 1.0); 
    (1.0-t) * Vec3::new(1.0,1.0,1.0) + t * Vec3::new(0.3,0.5,1.0)

}

fn main() {

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400; 
    let image_height = (image_width as f64 / aspect_ratio) as i64; 

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height; 
    let focal_length = 1.0; 

    let origin = Vec3::new(0.0,0.0,3.0); 
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0,viewport_height, 0.0);
    let llc = origin - (horizontal/2.0) - (vertical/2.0) - Vec3::new(0.0,0.0,focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height as i64).rev() {
        for i in 0..image_width as i64 {

            let u = i as f64 / image_width as f64; 
            let v = j as f64 / image_height as f64; 

            let dir = llc + (u* horizontal) + (v * vertical) - origin;

            let ray = Ray::new(origin, dir);

            let color = ray_color(ray);
            write_color(color); 

        }
    }
}



/* 
 * ========= ray ============= 
 */

#[derive(PartialEq, Clone, Copy, Debug)]
 struct Ray {
     pub orig: Vec3,
     pub dir: Vec3
 }

 impl Ray {

    pub fn new(origin: Vec3, dir: Vec3) -> Self {
         Self {
            orig: origin,
            dir: dir,
        }
    }

    pub fn at(self, t: f64) -> Vec3 {
       self.orig + (self.dir * t)
    }

}

/*
 * ===========================  
 */

