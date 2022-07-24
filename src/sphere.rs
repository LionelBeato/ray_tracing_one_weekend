use crate::vec3::Vec3; 
use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::hit::Hittable;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Sphere {
    pub center:Vec3,
    pub radius:f64,
}

impl Hittable for Sphere {
    fn hit (&self, r:&Ray, t_min:f64, t_max:f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(oc, r.dir);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (half_b*half_b) - a*c; 
        if discriminant < 0.0 { return None }
        let sqrtd = f64::sqrt(discriminant); 


        // find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a; 
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_min < root {
                return None
            }
        }

        let x = HitRecord 
        { 
            p: r.at(root), 
            normal: (r.at(root) - self.center) / self.radius,  
            t: root, 
            front_face: false, 
        };

        let mut rec: Option<HitRecord> = Some(x); 
        let outward_normal: Vec3 = (rec.as_ref().unwrap().p - self.center) / self.radius;
        rec.as_mut().unwrap().set_face_normal(r, outward_normal);

        rec
    }
}
