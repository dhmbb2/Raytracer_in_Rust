use crate::util::vec3::{Point3, Vec3};

#[derive(Clone, Debug)]
pub struct Ray {
    // origin
    pub ori: Point3,
    //direction, unitified
    pub dir: Vec3,
}

impl Ray {
    pub fn new(ori: Point3, dir: Vec3) -> Self {
        Self { ori, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.ori + self.dir * t
    }
}
