use super::HitRecord;
use super::Hittable;
use super::Point3;
use super::Vec3;
use super::Material;
use crate::util::bvh::AABB;
use crate::util::interval::Interval;
use crate::util::ray::Ray;

pub struct Quad<T: Material> {
    q: Point3,
    u: Vec3,
    v: Vec3,
    material: T,
    bbox: AABB,
}

impl<T: Material> Quad<T> {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: T) -> Self {
        let bbox = AABB::new_from_points(
            q - u - v,
            q + u + v,
        );
        Self { q, u, v, material, bbox }
    }
}

impl<T: Material> Hittable for Quad<T> {
    fn hit (&self, ray: &Ray, rot: &Interval) -> Option<HitRecord>{
        let normal = Vec3::cross(&self.u, &self.v);
        let d = Vec3::dot(&normal, &self.q);
        let t = (d - Vec3::dot(&normal, &ray.ori)) / Vec3::dot(&normal, &ray.dir);
        
        if t > rot.tmax || t < rot.tmin {
            return None;
        }

        let p = ray.at(t);
        let k = p - self.q;
        let w = normal / Vec3::dot(&normal, &normal);
        let a = Vec3::dot(&w, &Vec3::cross(&k, &self.v));
        let b = Vec3::dot(&w, &Vec3::cross(&self.u, &k));

        if a < 0.0 || a > 1.0 || b < 0.0 || b > 1.0 {
            return None;
        } else {
            return Some(HitRecord::new(
                p,
                t,
                normal.unit(),
                true,
                &self.material,
            ))
        }
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}