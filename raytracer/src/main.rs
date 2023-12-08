#![allow(clippy::float_cmp)]
#![feature(box_syntax)]

use std::time::{Duration, Instant};
use threadpool::ThreadPool;
use std::sync::mpsc;
use std::sync::Arc;
use image::Primitive;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;

pub mod camera;
pub mod hittable;
pub mod util;
pub mod world;
pub mod material;

use crate::world::World;
use camera::Camera;
use util::vec3::{Point3, Vec3};

// multi thread rendering only support bvh version
pub fn render_multi_thread(camera: Camera, n_jobs: usize, n_threads: usize) -> RgbImage{
    let (tx, rx) = mpsc::channel();
    let image_width = camera.image_width;
    let image_height = camera.image_height;
    let camera_ptr = Arc::new(camera);
    let pool = ThreadPool::new(n_threads);
    let bar = ProgressBar::new(n_jobs as u64);

    for k in 0..n_jobs {
        let tx = tx.clone();
        let camera_ptr = camera_ptr.clone();
        pool.execute(move || {
            let delta_height = camera_ptr.image_height / n_jobs as u32;
            let start_height = k as u32 * delta_height;
            let end_height = (k + 1) as u32 * delta_height;
            let mut _pixel = Point3::new(0.0, 0.0, 0.0);
            let mut frac: RgbImage = ImageBuffer::new(camera_ptr.image_width, delta_height);
            for j in start_height..end_height {
                for i in 0..camera_ptr.image_width{
                    _pixel = camera_ptr.pixel0_loc + camera_ptr.du * i as f64 + camera_ptr.dv * j as f64;
                    let color = camera_ptr.get_pixel_color(_pixel);
                    let color = Camera::linear_to_gamma(color);
                    let color = Camera::color2rgb(color);
                    frac.put_pixel(i, j-start_height, color)
                }
            }
            tx.send((start_height, end_height, frac))
                .expect("failed to send result");
        })
    }

    let mut result: RgbImage = ImageBuffer::new(image_width, image_height);

    for (start_height, end_height, frac) in rx.iter().take(n_jobs) {
        for j in start_height..end_height {
            for i in 0..image_width {
                let j = j as u32;
                let i = i as u32;
                *result.get_pixel_mut(i, j) = *frac.get_pixel(i, j-start_height);
            }
        }
        bar.inc(1);
    }
    bar.finish();
    result
}

fn main() {
    let center = Point3::new(-3.0,0.0, 1.0);
    let look_to = Vec3::new(0.0, 0.0, 0.0);
    let focal_length = 1.0;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let viewport_width = 2.0;
    let u = Vec3::new(0.0, -1.0, 0.0);
    let background_color = Vec3::new(0.0, 0.0, 0.0);

    let red_cloth= material::diffusive::Diffusive::new(Vec3::new(0.7, 0.3, 0.3));
    let grey_cloth = material::diffusive::Diffusive::new(Vec3::new(0.5, 0.5, 0.5));
    let silver_metal = material::metal::Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.1);
    let gold_metal = material::metal::Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0);
    let glass = material::dieletric::Dieletric::new(Vec3::new(1.0, 1.0, 1.0), 1.5);
    let light = material::light::Light::new(Vec3::new(3.0, 3.0, 3.0));
    let ball = hittable::sphere::Sphere::new(Vec3::new(2.0, 0.0, 0.0), 1.0, gold_metal.clone());
    let metal_ball = hittable::sphere::Sphere::new(Vec3::new(2.0, -1.5, 0.0), 0.4, silver_metal.clone());
    let glass_ball = hittable::sphere::Sphere::new(Vec3::new(1.0, 1.5, 0.3), 0.5, glass.clone());
    let ground_ball = hittable::sphere::Sphere::new(Vec3::new(0.0, 0.0, -1000.0), 999.0, silver_metal.clone());
    let light_ball = hittable::sphere::Sphere::new(Vec3::new(0.0, -33.0, 33.0), 29.0, light.clone());
    let light_quad =hittable::quad::Quad::new(Vec3::new(-1.0, 1.5, 1.5), Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, -1.5, 0.5), light.clone());

    let start = Instant::now();
    let mut world = World {
        hittables: vec![
            Box::new(ball), 
            Box::new(ground_ball), 
            Box::new(metal_ball), 
            Box::new(glass_ball),
            Box::new(light_ball),
            Box::new(light_quad),
            ],
    };

    for i in 0..30 {
        for j in 0..30{
            world.add_hittable(hittable::sphere::Sphere::new(Vec3::new(-15.0 + i as f64, -15.0 + j as f64, 0.5), 0.1, silver_metal.clone()));
        }
    }

    let camera = Camera::new(
        center,
        look_to,
        focal_length,
        aspect_ratio,
        image_width,
        viewport_width,
        u,
        world,
        background_color,
    );

    let picture: RgbImage = render_multi_thread(camera, 32, 4);
    // let picture: RgbImage = camera.render();
    let duration = start.elapsed();
    println!("Take {:?} to render!", duration);
    picture.save("output/test3.png").unwrap();
}
