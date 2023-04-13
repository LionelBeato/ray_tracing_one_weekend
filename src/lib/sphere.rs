use std::rc::Rc;

use crate::vec3::Vector3; 
use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::hit::Hittable;
use crate::material::Material;

// #[derive(PartialEq, Clone, Debug)]
pub struct Sphere {
    pub center:Vector3,
    pub radius:f64,
    // pub material_pointer: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit (&self, r:&Ray, t_min:f64, t_max:f64, mut rec:&mut HitRecord) -> bool {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = Vector3::dot(oc, &r.dir);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (half_b*half_b) - a*c; 
        if discriminant < 0.0 { return false }
        let sqrtd = f64::sqrt(discriminant); 


        // find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a; 
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_min < root {
                return false
            }
        }

        rec.t = root; 
        rec.p = r.at(root);
        let outward_normal: Vector3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        // rec.material_pointer = self.material_pointer.clone();

        true
    }
}

impl Sphere {
    pub fn new(center:Vector3, radius:f64) -> Self {
        Self {
           center: center,
           radius: radius,
       }
    }
}
