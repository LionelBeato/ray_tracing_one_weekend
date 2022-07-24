use crate::sphere::Sphere;
use crate::vec3::Vec3;
use crate::ray::Ray;
use std::cell::Cell;
use std::rc::Rc;
use std::vec::Vec; 

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face:bool,
}   

pub trait Hittable {
    fn hit (&self, r:&Ray, t_min:f64, t_max:f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r:&Ray, outward_normal:Vec3) {
        self.front_face = Vec3::dot(r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { 1.0 * outward_normal };          
    }
}

pub struct HittableList {   
   pub objects:Vec<Sphere>,
}

impl Hittable for HittableList {
    fn hit (&self, r:&Ray, t_min:f64, t_max:f64) -> Option<HitRecord> {

        let mut temp_rec:Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            match object.hit(r, t_min, closest_so_far) {
                Some(rec) => {
                    closest_so_far = rec.t;
                    temp_rec.replace(rec);
                }
                None => {}
            }
        }
        
        temp_rec
    }
}

impl HittableList {

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object:Sphere) {
        self.objects.push(object);
    }

}