use crate::{vec3::{Vector3, Point}, ray::Ray}; 

pub struct Camera {
    origin:Point,
    lower_left_corner:Point,
    horizontal:Vector3,
    vertical:Vector3,
    // u:Vector3, v:Vector3, w:Vector3,
    // lens_radius:f64,
} 


impl Camera {
    // pub fn new(look_from: Point, look_at: Point, vertical_up:Vector3, vertical_fov:f64, aperture:f64, focus_dist:f64, aspect_ratio:f64) -> Self {
    pub fn new() -> Self {
        // let theta:f64 = f64::to_radians(vertical_fov);
        // let h = f64::tan(theta / 2.0); 
        // let viewport_height:f64 = 2.0 * h;

        let aspect_ratio = 16.0 / 9.0;
        let viewport_height:f64 = 2.0;
        let viewport_width:f64 = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        // let w = (look_from - look_at).normalize();
        // let u = Vector3::cross(vertical_up, w).normalize();
        // let v = Vector3::cross(w, u);


        // let origin = look_from;
        // let horizontal = u * focus_dist * viewport_width;
        // let vertical = v * focus_dist * viewport_height;
        // let lower_left_corner = origin - (horizontal/2.0) - (vertical/2.0) - (w * focus_dist);

        let origin = Point::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical =  Vector3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - (horizontal/2.0) - (vertical/2.0) - Vector3::new(0.0, 0.0, focal_length);

        
        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, u:f64, v:f64) -> Ray {

        // let rd = Vector3::random_in_unit_disk() * self.lens_radius;
        // let offset = self.u * rd.x + self.v * rd.y; 

        // let or = self.origin + offset;
        // let dir = self.lower_left_corner + (self.horizontal * s) + (self.vertical * t) - self.origin - offset;


        Ray::new(
            self.origin, 
            self.lower_left_corner +
            self.horizontal * u +
            self.vertical * v
            - self.origin
        )  
    }
}

