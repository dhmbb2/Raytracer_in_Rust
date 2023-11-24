use super::HitRecord;
use super::Hittable;
use super::Vec3;
use super::Point3;

pub struct Sphere{
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self { Self { center, radius } }
}

impl Hittable for Sphere {
    fn hit(
        &self, 
        ray: &crate::ray::Ray, 
        rot: &crate::util::ROT, 
    ) -> Option<HitRecord> {
        // calculate the distance between ray origin and sphere center
        let v = self.center - ray.ori;
        let mid: f64 = Vec3::dot(&v, &ray.dir);
        let distance = (v.squared_length()- mid.powi(2)).sqrt();

        if distance > self.radius {
            return None;
        }

        let dt = (self.radius.powi(2) - distance.powi(2)).sqrt();
        let t_1 = mid - dt;
        let t_2 = mid + dt;

        if rot.in_between_open(t_1)  {
            Some( HitRecord::new(
                ray.at(t_1),
                t_1,
                (ray.at(t_1) - self.center) / self.radius,
                true,
            ))
        } else if rot.in_between_open(t_2) {
            Some( HitRecord::new(
                ray.at(t_2),
                t_2,
                (ray.at(t_2) - self.center) / self.radius,
                false,
            ))
        } else {
            None
        }
        
        
    }
}