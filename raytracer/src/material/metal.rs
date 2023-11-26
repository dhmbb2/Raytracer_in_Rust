use crate::util::ray::Ray;
use crate::util::vec3::{Vec3, Color};

use super::Material;
use crate::hittable::HitRecord;

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Ray {
        let scatter_direction_reflect = ray.dir - hit_record.normal * 2.0 * Vec3::dot(&ray.dir, &hit_record.normal);
        let scatter_direction = scatter_direction_reflect + Vec3::random_unit() * self.fuzz;
        if scatter_direction.near_zero() {
            return Ray::new(hit_record.point, hit_record.normal);
        }
        Ray::new(hit_record.point, scatter_direction.unit())
    }

    fn attenuation(&self) -> Color {
        self.albedo
    }
}