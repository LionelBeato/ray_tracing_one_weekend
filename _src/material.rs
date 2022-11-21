use crate::{ray::Ray, hit::HitRecord, vec3::Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, attenuation:&mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct Metal {
    pub albedo:Vec3,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, attenuation:&mut Vec3, scattered: &mut Ray) -> bool {

        let reflected = Vec3::reflect(Vec3::unit_vector(ray.dir), hit_record.normal);
        // scattered = &mut Ray::new(hit_record.p, reflected);
        // *attenuation = self.albedo;
        Vec3::dot(scattered.dir, reflected) > 0.0

    }
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub struct Lambertian {
    pub albedo:Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, attenuation:&mut Vec3, mut scattered:&mut Ray) -> bool {


        let mut scattered_direction = hit_record.normal + Vec3::random_unit_vector(); 


        if scattered_direction.near_zero() {
            scattered_direction = hit_record.normal;
        }


        *scattered = Ray::new(hit_record.p, scattered_direction);
        *attenuation = self.albedo;
        true
    }
    
}

