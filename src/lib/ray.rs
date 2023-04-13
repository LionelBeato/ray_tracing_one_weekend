use vec3::Vector3;

use crate::vec3;

#[derive(PartialEq, Clone, Copy, Debug)]
 pub struct Ray {
     pub orig: Vector3,
     pub dir: Vector3
 }

 impl Ray {

    pub fn new(origin: Vector3, dir: Vector3) -> Self {
         Self {
            orig: origin,
            dir: dir,
        }
    }

    pub fn at(self, t: f64) -> Vector3 {
       self.orig + (self.dir * t)
    }

}