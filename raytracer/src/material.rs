pub mod diffusive;
pub mod metal;
pub mod dieletric; 
pub mod light;

use crate::util::ray::Ray;
use crate::hittable::HitRecord;
use crate::util::vec3::Color;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Ray {
        Ray::new(hit_record.point, ray.dir)
    }

    fn attenuation(&self) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn is_light(&self) -> bool {
        false
    }
}