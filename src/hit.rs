use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face:bool,
}   

pub trait Hittable {
    fn hit (r:Ray, t_min:f64, t_max:f64, rec:HitRecord) -> bool;
}

impl HitRecord {
    pub fn set_face_normal(&self, r:Ray, outward_normal:Vec3) {
        self.front_face = dot(r.dir, outward_normal) < 0;
        self.normal = if front_face { outward_normal } else { -outward_normal };          
    }
}

pub struct HittableList {   
   
}

impl Hittable for HittableList {
    
}