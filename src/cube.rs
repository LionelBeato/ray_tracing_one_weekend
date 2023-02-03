use crate::vec3::Vector3; 
use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::hit::Hittable;
use crate::material::Material;

pub struct Cube {
    pub center:f64,
    pub material_pointer: Rc<dyn Material>,
}