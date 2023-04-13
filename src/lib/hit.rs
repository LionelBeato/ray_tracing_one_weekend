use crate::sphere::Sphere;
use crate::vec3::Vector3;
use crate::ray::Ray;
use crate::material::{Material, Metal, Lambertian};
use std::borrow::Borrow;
use std::cell::Cell;
use std::rc::Rc;
use std::vec::Vec; 


#[derive(Clone)]
pub struct HitRecord {
    pub p: Vector3,
    pub normal: Vector3,
    pub material_pointer: Rc<dyn Material>,
    pub t: f64,
    pub front_face:bool,
}   

pub trait Hittable {
    fn hit (&self, r:&Ray, t_min:f64, t_max:f64, rec:&mut HitRecord) -> bool;
}

impl HitRecord {
    #[inline(always)]
    pub fn set_face_normal(&mut self, r:&Ray, outward_normal:Vector3) {
        self.front_face = Vector3::dot(r.dir, &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { outward_normal * -1.0 };          
    }
}
#[derive(Clone)]
pub struct HittableList {   
   pub objects:Vec<Rc<Sphere>>,
}

impl Hittable for HittableList {
    fn hit (&self, r:&Ray, t_min:f64, t_max:f64, rec:&mut HitRecord) -> bool {

        let mut hit_anything = false;

        let mut temp_rec = &mut HitRecord {
            p:Vector3::random(), 
            normal: Vector3::random(),
            material_pointer: Rc::new(Lambertian { albedo: Vector3::random() }), 
            t: 0.0, 
            front_face: false
        };

        let mut closest_so_far = t_max;

        for object in &self.objects {

            if object.hit(r, t_min, closest_so_far, temp_rec) {
                 
                    hit_anything = true;
                    closest_so_far = temp_rec.clone().t;
                    *rec = temp_rec.clone();
                    
                }
        }  
        return hit_anything;
    }
}

impl HittableList {

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object:Rc<Sphere>) {
        self.objects.push(object);
    }

}