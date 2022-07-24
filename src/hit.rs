use crate::vec3::Vec3;
use crate::ray::Ray;
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
    fn hit (&self, r:Ray, t_min:f64, t_max:f64, rec:HitRecord) -> bool;
}

impl HitRecord {
    pub fn set_face_normal(mut self, r:Ray, outward_normal:Vec3) {
        self.front_face = Vec3::dot(r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -1.0 * outward_normal };          
    }
}

pub struct HittableList {   
   pub objects:Vec<Rc<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit (&self, r:Ray, t_min:f64, t_max:f64, mut rec:HitRecord) -> bool {
        let temp_rec:HitRecord = HitRecord { p: Vec3::new(0.0, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 0.0), t: 0.0, front_face: false }; 
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, temp_rec) {
                hit_anything = true; 
                closest_so_far = temp_rec.t; 
                rec = temp_rec;
            }        
        }
        hit_anything
    }
}

impl HittableList {

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object:Rc<dyn Hittable>) {
        self.objects.push(object);
    }

}