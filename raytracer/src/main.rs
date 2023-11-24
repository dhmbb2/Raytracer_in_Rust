#![allow(clippy::float_cmp)]
#![feature(box_syntax)]

use image::RgbImage;

pub mod camera;
pub mod hittable;
pub mod util;
pub mod world;

use crate::world::World;
use camera::Camera;
use util::vec3::{Point3, Vec3};

fn main() {
    let center = Point3::new(-1.0, 0.0, 0.0);
    let direction = Vec3::new(1.0, 0.0, 0.0);
    let focal_length = 1.0;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let viewport_width = 2.0;
    let u = Vec3::new(0.0, -1.0, 0.0);

    let ball = hittable::sphere::Sphere::new(Vec3::new(2.0, 0.0, 0.0), 1.0);
    let ground_ball = hittable::sphere::Sphere::new(Vec3::new(0.0, 0.0, -100.0), 98.0);

    let world = World {
        hittables: vec![Box::new(ball), Box::new(ground_ball)],
    };

    let camera = Camera::new(
        center,
        direction,
        focal_length,
        aspect_ratio,
        image_width,
        viewport_width,
        u,
        world,
    );

    let picture: RgbImage = camera.render();
    picture.save("output/test.png").unwrap();
}
