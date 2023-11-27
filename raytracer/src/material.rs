pub mod diffusive;
pub mod metal;
pub mod dieletric; 

use crate::util::ray::Ray;
use crate::hittable::HitRecord;
use crate::util::vec3::Color;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Ray;

    fn attenuation(&self) -> Color;
}

