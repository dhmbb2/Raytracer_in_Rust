#![allow(clippy::float_cmp)]
#![feature(box_syntax)]

use image::RgbImage;
pub mod camera;
pub mod hittable;
pub mod util;
pub mod world;
pub mod material;

use crate::world::World;
use camera::Camera;
use util::vec3::{Point3, Vec3};

fn main() {
    let center = Point3::new(-2.0, 0.0, 0.0);
    let look_to = Vec3::new(2.0, 0.0, 0.0);
    let focal_length = 1.0;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let viewport_width = 2.0;
    let u = Vec3::new(0.0, -1.0, 0.0);

    let red_cloth= material::diffusive::Diffusive::new(Vec3::new(0.7, 0.3, 0.3));
    let grey_cloth = material::diffusive::Diffusive::new(Vec3::new(0.5, 0.5, 0.5));
    let silver_metal = material::metal::Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3);
    let gold_metal = material::metal::Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0);
    let glass = material::dieletric::Dieletric::new(Vec3::new(1.0, 1.0, 1.0), 1.5);
    let ball = hittable::sphere::Sphere::new(Vec3::new(2.0, 0.0, 0.0), 1.0, gold_metal.clone());
    let metal_ball = hittable::sphere::Sphere::new(Vec3::new(2.0, -1.5, 0.0), 0.4, silver_metal.clone());
    let left_ball = hittable::sphere::Sphere::new(Vec3::new(0.0, 0.5, 0.3), 0.5, glass.clone());
    let ground_ball = hittable::sphere::Sphere::new(Vec3::new(0.0, 0.0, -100.0), 99.0, grey_cloth.clone());

    let world = World {
        hittables: vec![Box::new(ball), Box::new(ground_ball), Box::new(metal_ball), Box::new(left_ball)],
    };

    let camera = Camera::new(
        center,
        look_to,
        focal_length,
        aspect_ratio,
        image_width,
        viewport_width,
        u,
        world,
    );

    let picture: RgbImage = camera.render();
    picture.save("output/test2.png").unwrap();
}
