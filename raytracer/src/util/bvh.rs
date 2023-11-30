use crate::util::interval::Interval;
use crate::util::ray::Ray;
use crate::util::vec3::Point3;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::world::World;

use rand::Rng;

#[derive(Debug, Clone, Copy)]
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
            x: Interval::new(f64::min(p1.x, p2.x),f64::max(p1.x, p2.x)),
            y: Interval::new(f64::min(p1.y, p2.y),f64::max(p1.y, p2.y)),
            z: Interval::new(f64::min(p1.z, p2.z), f64::max(p1.z, p2.z)),
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
    pub fn hit(&self, ray: &Ray, rot: &Interval) -> bool {
        let mut ray_t_min = rot.tmin;
        let mut ray_t_max = rot.tmax; 
        for i in 0..3 {
            let t_0 = f64::min((self.get_axis(i).tmin - ray.ori.get_axis(i)) / ray.dir.get_axis(i), 
                               (self.get_axis(i).tmax - ray.ori.get_axis(i)) / ray.dir.get_axis(i));
            let t_1 = f64::max((self.get_axis(i).tmin - ray.ori.get_axis(i)) / ray.dir.get_axis(i), 
                               (self.get_axis(i).tmax - ray.ori.get_axis(i)) / ray.dir.get_axis(i));
            ray_t_min = f64::max(ray_t_min, t_0);
            // println!("{}: {}", i, ray_t_min);
            ray_t_max = f64::min(ray_t_max, t_1);
            // println!("{}: {}", i, ray_t_max);
            if ray_t_max <= ray_t_min {
                return false;
            }
        }
        return true;
    } 
}


pub struct BVHNode {
    pub left: Option<Box<BVHNode>>,
    pub right: Option<Box<BVHNode>>,
    pub obj: Option<Box<dyn Hittable>>,
    pub bbox: AABB,
}

impl BVHNode {
    pub fn new(left: Option<Box<BVHNode>>, right: Option<Box<BVHNode>>, obj: Option<Box<dyn Hittable>>) -> Self {
        let bbox = match (&left, &right, &obj) {
            (Some(l), Some(r), None) => AABB::new_from_aabb(&l.bbox(), &r.bbox()),
            (None, None,Some(obj)) => obj.bbox(),
            _ => panic!("Invalid BVHNode"),
        };
        Self { left, right, obj, bbox }
    }

    pub fn new_from_world(world: World) -> Self {
        Self::new_from_vec(world.hittables)
    }

    pub fn new_from_vec(mut hittables: Vec<Box<dyn Hittable>>) -> Self {
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0..3);
        let length = hittables.len();
        hittables.sort_by(|a, b| {
            let a_bbox = a.bbox();
            let b_bbox = b.bbox();
            a_bbox.get_axis(axis).tmin.partial_cmp(&b_bbox.get_axis(axis).tmin).unwrap()
        });
        if length == 1 {
            let hittable = hittables.pop().unwrap();
            Self::new(None, None, Some(hittable))
        } else {
            let mut left_vec = hittables;
            let right_vec = left_vec.split_off(length / 2);
            let left = Some(Box::new(Self::new_from_vec(left_vec)));
            let right = Some(Box::new(Self::new_from_vec(right_vec)));
            // if let Some(l) = &left {
            //     println!("l: {} {:?}", length/2, l.bbox);
            // }
            // if let Some(r) = &right {
            //     println!("r: {} {:?}", (length as f64 / 2.0 + 0.5) as i32, r.bbox);
            // }
            Self::new(left, right, None)
        }
    }

}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, rot: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, rot) {
            return None;
        }
        match (&self.left, &self.right, &self.obj) {
            (None, None, Some(obj)) => {
                return obj.hit(ray, rot);
            }
            (Some(l), Some(r), None) => {
                let hit_record_l = l.hit(ray, rot);
                let mut rot = rot.clone();
                if let Some(record) = hit_record_l {
                    rot.set_tmax(record.t);
                }
                let hit_record_r = r.hit(ray, &rot);
                if let Some(record) = hit_record_r {
                    Some(record)
                } else {
                    hit_record_l
                }
            }
            _ => panic!("Invalid BVHNode during hitting")
        }
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}