use crate::util::ray::Ray;
use crate::util::vec3::{Vec3, Color};

use super::Material;
use crate::hittable::HitRecord;

#[derive(Clone, Copy)]
pub struct Diffusive {
    pub albedo: Color,
}

impl Diffusive {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Diffusive {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Ray {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        Ray::new(hit_record.point, scatter_direction.unit())
    }

    fn attenuation(&self) -> Color {
        self.albedo
    }
}