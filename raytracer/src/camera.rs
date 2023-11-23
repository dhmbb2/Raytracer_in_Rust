use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

use std::default::Default;

#[derive(Debug)]
pub struct Camera {
    // user specified parameters
    center: Point3,
    direction: Vec3,
    focal_length: f64,
    aspect_ratio: f64,
    image_width: u64,
    viewport_width: f64,
    u: Vec3, // the u axis of the viewport, from left to right

    // detailed paras for the camera
    image_height: u64,
    viewport_height: f64,
    pixel_length: f64,
    pixel0_loc: Point3,
    du: Vec3, // unit pixel vector of u axis
    dv: Vec3, // unit pixel vector of v axis
}

impl Camera {
    pub fn new(
        center: Point3,
        direction: Vec3,
        focal_length: f64,
        aspect_ratio: f64,
        image_width: u64,
        viewport_width: f64,
        u: Vec3,
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u64;
        let viewport_height = viewport_width / image_width as f64 * image_height as f64;
        let pixel_length = viewport_width / image_width as f64;

        let u_unit = u.unit();
        let v_unit = Vec3::cross(&direction, &u_unit).unit();

        let left_corner: Point3 = center + direction.unit() * focal_length
            - u_unit * (0.5 * viewport_width)
            - v_unit * (0.5 * viewport_height);
        let pixel0_loc = left_corner + u_unit * 0.5 * pixel_length + v_unit * 0.5 * pixel_length;

        let du = u_unit * pixel_length;
        let dv = v_unit * pixel_length;

        return Self {
            center,
            direction,
            focal_length,
            aspect_ratio,
            image_width,
            viewport_width,
            u,
            image_height,
            viewport_height,
            pixel_length,
            pixel0_loc,
            du,
            dv,
        };
    }

    pub fn cast_ray(&self, pixel_loc: Point3) -> Ray {
        let ray_direction = pixel_loc - self.center;
        return Ray::new(self.center, ray_direction);
    }

    pub fn get_color(ray: Ray) -> Color {
        let unit_direction: Vec3 = ray.dir.unit();
        let a = 0.5 * (unit_direction.y + 1.0);
        return Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a;
    }

    pub fn render(&self) {
        let mut _pixel = Point3::new(0.0, 0.0, 0.0);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                _pixel = self.pixel0_loc + self.du * i as f64 + self.dv * j as f64;
                let ray = self.cast_ray(_pixel);
                let color: Color = Self::get_color(ray);
                println!(
                    "{} {} {}",
                    (color.x * 255.999) as u64,
                    (color.y * 255.999) as u64,
                    (color.z * 255.999) as u64
                );
            }
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        let center = Point3::new(-1.0, 0.0, 0.0);
        let direction = Vec3::new(1.0, 0.0, 0.0);
        let focal_length = 1.0;
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
        let viewport_width = 2.0;
        let u = Vec3::new(0.0, 1.0, 0.0);

        return Self::new(
            center,
            direction,
            focal_length,
            aspect_ratio,
            image_width,
            viewport_width,
            u,
        );
    }
}
