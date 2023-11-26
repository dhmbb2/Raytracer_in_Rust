use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;

use crate::hittable::HitRecord;
use crate::util::const_value;
use crate::util::ray::Ray;
use crate::util::rot::ROT;
use crate::util::vec3::{Color, Point3, Vec3};
use crate::world::World;

pub struct Camera {
    // user specified parameters
    center: Point3,
    look_to: Point3,
    focal_length: f64,
    aspect_ratio: f64,
    image_width: u32,
    viewport_width: f64,
    u: Vec3,      // the u axis of the viewport, from left to right
    world: World, // the world of which we capture

    // detailed paras for the camera
    image_height: u32,
    viewport_height: f64,
    pixel_length: f64,
    pixel0_loc: Point3,
    du: Vec3, // unit pixel vector of u axis
    dv: Vec3, // unit pixel vector of v axis
}

impl Camera {
    pub fn new(
        center: Point3,
        look_to: Vec3,
        focal_length: f64,
        aspect_ratio: f64,
        image_width: u32,
        viewport_width: f64,
        u: Vec3,
        world: World,
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let viewport_height = viewport_width / image_width as f64 * image_height as f64;
        let pixel_length = viewport_width / image_width as f64;
        let direction = look_to - center;

        let u_unit = u.unit();
        let v_unit = Vec3::cross(&direction, &u_unit).unit();

        let left_corner: Point3 = center + direction.unit() * focal_length
            - u_unit * (0.5 * viewport_width)
            - v_unit * (0.5 * viewport_height);
        let pixel0_loc = left_corner + u_unit * 0.5 * pixel_length + v_unit * 0.5 * pixel_length;

        let du = u_unit * pixel_length;
        let dv = v_unit * pixel_length;

        Self {
            center,
            look_to,
            focal_length,
            aspect_ratio,
            image_width,
            viewport_width,
            u,
            world,
            image_height,
            viewport_height,
            pixel_length,
            pixel0_loc,
            du,
            dv,
        }
    }

    fn linear_to_gamma(&self, color: Color) -> Color {
        Color::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt())
    }

    pub fn cast_ray(&self, pixel_loc: &Point3) -> Ray {
        let ray_direction = (pixel_loc.clone() - self.center).unit();
        return Ray::new(self.center, ray_direction);
    }

    pub fn get_color(&self, ray: Ray, bounce_time: u32) -> Color {
        let a = 0.5 * (ray.dir.y + 1.0);
        let bounce_time = bounce_time + 1;
        let mut _hit_record: Option<HitRecord> = None;
        if bounce_time > const_value::MAX_BOUNCING_TIMES {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rot = ROT::new(const_value::BACKGROUND_T, 0.0);
        for hittable in &self.world.hittables {
            if let Some(record) = hittable.hit(&ray, &rot) {
                if record.t < 0.001 {
                    continue;
                }
                _hit_record = Some(record);
                rot.set_tmax(record.t);
            }
        }

        match _hit_record {
            None => Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a,
            Some(hit_record) => {
                let scattered_ray = hit_record.material.scatter(&ray, &hit_record);
                hit_record.material.attenuation() * self.get_color(scattered_ray, bounce_time)
            }
        }
    }

    pub fn get_pixel_color(&self, pixel_loc: Point3) -> Color {
        let mut rng = rand::thread_rng();
        let mut color: Color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..const_value::RAY_PER_PIXEL {
            let u_offset: f64 = rng.gen::<f64>() - 0.5;
            let v_offset: f64 = rng.gen::<f64>() - 0.5; 
            let pixel_loc_new = pixel_loc + self.du * u_offset + self.dv * v_offset;
            let ray = self.cast_ray(&pixel_loc_new);
            let bounce_times = 0;
            color += self.get_color(ray, bounce_times);
        }
        color / const_value::RAY_PER_PIXEL as f64
    }

    pub fn render(&self) -> RgbImage {
        let mut _pixel = Point3::new(0.0, 0.0, 0.0);
        let mut result: RgbImage = ImageBuffer::new(self.image_width, self.image_height);
        let bar = ProgressBar::new((self.image_height * self.image_width) as u64);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                _pixel = self.pixel0_loc + self.du * i as f64 + self.dv * j as f64;
                let color = self.get_pixel_color(_pixel);
                let color = self.linear_to_gamma(color);
                result.put_pixel(
                    i,
                    j,
                    Rgb([
                        (color.x * 255.999) as u8,
                        (color.y * 255.999) as u8,
                        (color.z * 255.999) as u8,
                    ]),
                );
                bar.inc(1);
            }
        }
        bar.finish();
        result
    }
}
