use crate::util::interval::Interval;
use crate::util::ray::Ray;
use crate::util::vec3::Point3;
use crate::util::const_value;
use crate::hittable::Hittable;

#[derive(Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    // Generate AABB from two diagnal points
    pub fn new_from_points(p1: Point3, p2: Point3) -> Self {
        Self {
            x: Interval::new(f64::max(p1.x, p2.x), f64::min(p1.x, p2.x)),
            y: Interval::new(f64::max(p1.y, p2.y), f64::min(p1.y, p2.y)),
            z: Interval::new(f64::max(p1.z, p2.z), f64::min(p1.z, p2.z)),
        }
    }

    // Merge two AABBs
    pub fn new_from_aabb(op1: &Self, op2: &Self) -> Self {
        let x = Interval::merge(&op1.x, &op2.x);
        let y = Interval::merge(&op1.y, &op2.y);
        let z = Interval::merge(&op1.z, &op2.z);
        Self { x, y, z }
    }

    // return interval of the given axis
    pub fn get_axis(&self, axis: usize) -> Interval {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid axis"),
        }
    }

    // return true if the ray hit the AABB
    pub fn hit(&self, ray: &Ray) -> bool {
        let mut ray_t_min:f64 = -1.0;
        let mut ray_t_max:f64 = const_value::BACKGROUND_T; 
        for i in 0..3 {
            let t_0 = f64::min(self.get_axis(i).tmin - ray.ori.get_axis(i), 
                               self.get_axis(i).tmax - ray.ori.get_axis(i)) / ray.dir.get_axis(i);
            let t_1 = f64::max(self.get_axis(i).tmin - ray.ori.get_axis(i), 
                               self.get_axis(i).tmax - ray.ori.get_axis(i)) / ray.dir.get_axis(i);
            ray_t_min = f64::max(ray_t_min, t_0);
            ray_t_max = f64::min(ray_t_max, t_1);
            if ray_t_max <= ray_t_min {
                return false;
            }
        }
        return true;
    } 
}


pub struct BVHNode {
    pub left: Option<Box<dyn Hittable>>,
    pub right: Option<Box<dyn Hittable>>,
    pub bbox: AABB,
}

impl BVHNode {
    pub fn new(left: Option<Box<dyn Hittable>>, right: Option<Box<dyn Hittable>>) -> Self {
        let bbox = match (&left, &right) {
            (Some(l), Some(r)) => AABB::new_from_aabb(&l.bbox(), &r.bbox()),
            (None, None) => panic!("Invalid BVHNode"),
            _ => panic!("Invalid BVHNode"),
        };
        Self { left, right, bbox }
    }

    pub fn new_from_vec() {
        // TODO
    }
}