use crate::util::ray::Ray;
use crate::util::vec3::{Vec3, Color};
use rand::Rng;

use super::Material;
use crate::hittable::HitRecord;

#[derive(Clone, Copy)]
pub struct Dieletric {
    pub albedo: Color,
    pub ita: f64,
}

impl Dieletric {
    pub fn new(albedo: Color, ita: f64) -> Self {
        Self { albedo, ita }
    }

    fn reflect(&self, ray: &Ray, hit_record: &HitRecord) -> Vec3 {
        let reflected_dir = ray.dir - hit_record.normal * 2.0 * Vec3::dot(&ray.dir, &hit_record.normal);
        if reflected_dir.near_zero() {
            return hit_record.normal;
        }
        reflected_dir.unit()
    }

    fn refract(&self, ray: &Ray, hit_record: &HitRecord, refraction_ratio: f64) -> Vec3 {
        let cos_theta = -Vec3::dot(&ray.dir, &hit_record.normal);

        let r_parallel = (ray.dir + hit_record.normal * cos_theta) * refraction_ratio;
        let r_perpendicular = -hit_record.normal * (1.0 - r_parallel.squared_length()).sqrt();
        
        return r_parallel + r_perpendicular;

    }

    fn reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
        let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }

}

impl Material for Dieletric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Ray {
        let cos_theta = -Vec3::dot(&ray.dir, &hit_record.normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        
        let refraction_ratio = if hit_record.is_outward {
            1.0 / self.ita
        } else {
            self.ita
        };

        let mut rng  = rand::thread_rng();

        let cannot_refract = refraction_ratio * sin_theta > 1.0 || Self::reflectance(cos_theta, refraction_ratio) > rng.gen();

        let direction = if cannot_refract {
            self.reflect(&ray, &hit_record)
        } else {
            self.refract(&ray, &hit_record, refraction_ratio)
        };

        Ray::new(hit_record.point, direction)
    }

    fn attenuation(&self) -> Color {
        self.albedo
    }
}