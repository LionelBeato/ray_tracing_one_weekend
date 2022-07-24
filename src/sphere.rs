use crate::vec3::Vec3; 
use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::hit::Hittable;

pub struct Sphere {
    pub center:Vec3,
    pub radius:f64,
}

impl Hittable for Sphere {
    fn hit (r:Ray, t_min:f64, t_max:f64, rec:HitRecord) {
        let oc = r.orig - center;
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(oc, r.dir);
        let c = oc.length_squared() - (radius * radius);

        let discriminant = (half_b*half_b) - a*c; 
        if (discriminant < 0.0) { false }
        let sqrtd = f64::sqrt(discriminant); 

        // find the nearest root that lies in the acceptable range.
        let root = (-half_b - sqrtd) / a; 
        if (root < t_min || t_max < root) {
            root = (-half_b + sqrtd) / a;
            if (root < t_min || t_min < root) {
                false
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p -center) / radius; 
        let outward_normal:Vec3 = (rec.p - center) / radius;
        rec.set_face_normal(r, outward_normal); 

        true
    }
}
