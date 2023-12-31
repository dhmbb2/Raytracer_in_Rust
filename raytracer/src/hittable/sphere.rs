use super::HitRecord;
use super::Hittable;
use super::Point3;
use super::Vec3;
use super::Material;
use crate::util::bvh::AABB;
use crate::util::interval::Interval;
use crate::util::ray::Ray;

pub struct Sphere<T: Material> {
    center: Point3,
    radius: f64,
    material: T,
    bbox: AABB,
}

impl<T: Material> Sphere<T> {
    pub fn new(center: Point3, radius: f64, material: T) -> Self {
        let bbox = AABB::new_from_points(
            center - Vec3::new(radius, radius, radius),
            center + Vec3::new(radius, radius, radius),
        );
        Self { center, radius, material, bbox }
    }
}

impl<T: Material> Hittable for Sphere<T> {
    fn hit(
        &self, 
        ray: &Ray, 
        rot: &Interval
    ) -> Option<HitRecord> {
        // calculate the distance between ray origin and sphere center
        let v = self.center - ray.ori;
        let mid: f64 = Vec3::dot(&v, &ray.dir);
        let distance = (v.squared_length() - mid.powi(2)).sqrt();

        if distance > self.radius {
            return None;
        }

        let dt = (self.radius.powi(2) - distance.powi(2)).sqrt();
        let t_1 = mid - dt;
        let t_2 = mid + dt;

        if rot.in_between_open(t_1) {
            Some(HitRecord::new(
                ray.at(t_1),
                t_1,
                (ray.at(t_1) - self.center) / self.radius,
                true,
                &self.material,
            ))
        } else if rot.in_between_open(t_2) {
            Some(HitRecord::new(
                ray.at(t_2),
                t_2,
                (ray.at(t_2) - self.center) / self.radius,
                false,
                &self.material,
            ))
        } else {
            None
        }
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}
