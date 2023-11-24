pub mod sphere;

use crate::vec3::{Vec3, Point3};
use crate::util::ROT;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray, rot: &ROT) -> Option<HitRecord>;

}
#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub t: f64,
    pub normal: Vec3, // normal vector of the hit point
    pub is_outward: bool,
}

impl HitRecord {
    pub fn new(point: Point3, t: f64, normal: Vec3, is_outward: bool) -> Self { 
        Self::reset_normal(normal, is_outward);
        Self { point, t, normal, is_outward } 
    }

    fn reset_normal(normal: Vec3, is_outward: bool) -> Vec3 {
        if is_outward {normal} else {-normal}
    }
}

