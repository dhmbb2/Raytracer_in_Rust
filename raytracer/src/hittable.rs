pub mod sphere;

use crate::util::ray::Ray;
use crate::util::interval::Interval;
use crate::util::vec3::{Point3, Vec3};
use crate::material::Material;
use crate::util::bvh::AABB;


pub trait Hittable {
    fn hit(&self, ray: &Ray, rot: &Interval) -> Option<HitRecord>;

    fn bbox(&self) -> AABB;

    
    fn get_center(&self) -> Point3 {
        Point3::new(0.0, 0.0, 0.0)
    }
}
#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub point: Point3,
    pub t: f64,
    pub normal: Vec3, // normal vector of the hit point
    pub is_outward: bool,
    pub material: &'a dyn Material
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Point3, 
        t: f64, normal: Vec3, 
        is_outward: bool,
        material: &'a dyn Material,
    ) -> Self {
        let normal = Self::reset_normal(normal, is_outward);
        Self {
            point,
            t,
            normal,
            is_outward,
            material,
        }
    }

    fn reset_normal(normal: Vec3, is_outward: bool) -> Vec3 {
        if is_outward {
            normal
        } else {
            -normal
        }
    }

}

