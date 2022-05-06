use crate::vec3::Vec3;

#[derive(PartialEq, Clone, Copy, Debug)]
 pub struct Ray {
     pub orig: Vec3,
     pub dir: Vec3
 }

 impl Ray {

    pub fn new(origin: Vec3, dir: Vec3) -> Self {
         Self {
            orig: origin,
            dir: dir,
        }
    }

    pub fn at(self, t: i64) -> Vec3 {
       self.orig + (self.dir * t)
    }

}