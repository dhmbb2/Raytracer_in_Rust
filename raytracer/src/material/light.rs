use crate::util::ray::Ray;
use crate::util::vec3::{Vec3, Color};

use super::Material;
use crate::hittable::HitRecord;

#[derive(Clone, Copy)]
pub struct Light {
    pub light_color: Color,
}

impl Light {
    pub fn new(light_color: Color) -> Self {
        Self { light_color }
    }
}

impl Material for Light {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Ray {
        Ray::new(hit_record.point, Vec3::new(0.0, 0.0, 0.0))
    }

    fn attenuation(&self) -> Color {
        return self.light_color;
    }

    fn is_light(&self) -> bool {
        true
    }
}