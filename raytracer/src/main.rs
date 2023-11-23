#![allow(clippy::float_cmp)]
#![feature(box_syntax)]

pub mod camera;
pub mod ray;
pub mod vec3;

use camera::Camera;

fn main() {
    println!("P3\n400 200\n255");
    let camera: Camera = Default::default();
    camera.render();
}
