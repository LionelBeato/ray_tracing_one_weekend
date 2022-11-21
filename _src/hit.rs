use crate::sphere::Sphere;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::{Material, Metal, Lambertian};
use std::borrow::Borrow;
use std::cell::Cell;
use std::rc::Rc;
use std::vec::Vec; 


#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub material_pointer: Rc<dyn Material>,
    pub t: f32,
    pub front_face:bool,
}   

pub trait Hittable {
    fn hit (&self, r:&Ray, t_min:f32, t_max:f32, rec:&mut HitRecord) -> bool;
}

impl HitRecord {
    #[inline(always)]
    pub fn set_face_normal(&mut self, r:&Ray, outward_normal:Vec3) {
        self.front_face = Vec3::dot(r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -1.0 * outward_normal };          
    }
}
#[derive(Clone)]
pub struct HittableList {   
   pub objects:Vec<Rc<Sphere>>,
}

impl Hittable for HittableList {
    fn hit (&self, r:&Ray, t_min:f32, t_max:f32, rec:&mut HitRecord) -> bool {

        let mut hit_anything = false;

        let mut temp_rec = &mut HitRecord {
            p:Vec3::random(), 
            normal: Vec3::random(),
            material_pointer: Rc::new(Lambertian { albedo: Vec3::random() }), 
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