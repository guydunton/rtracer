use super::{world::WorldImpl, Ray};
use crate::canvas::{Canvas, Color};
use crate::maths::{Matrix4x4, Point};

use rayon::prelude::*;

pub struct Camera {
    width: i32,
    height: i32,
    fov: f64,
    transform: Matrix4x4,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
    transform_inverse: Matrix4x4,
}

impl Camera {
    pub fn new(width: i32, height: i32, fov: f64, transform: Matrix4x4) -> Self {
        let (pixel_size, half_width, half_height) = Self::calculate_pixel_size(width, height, fov);
        Self {
            width,
            height,
            fov,
            transform,
            pixel_size,
            half_width,
            half_height,
            transform_inverse: transform.inverse().unwrap(),
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn fov(&self) -> f64 {
        self.fov
    }

    pub fn transform(&self) -> Matrix4x4 {
        self.transform
    }

    fn calculate_pixel_size(width: i32, height: i32, fov: f64) -> (f64, f64, f64) {
        let half_view = (fov / 2.0).tan();
        let aspect = width as f64 / height as f64;

        let half_width;
        let half_height;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / width as f64;

        (pixel_size, half_width, half_height)
    }

    pub fn pixel_size(&self) -> f64 {
        self.pixel_size
    }

    pub fn ray_for_pixel(&self, x: i32, y: i32) -> Ray {
        // The offset from the edge of the canvas to the pixels center
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        // The untransformed coordinates of the pixel in world space.
        // (remember that the camera looks towards -z. so +x is to the *left*).
        let worldx = self.half_width - xoffset;
        let worldy = self.half_height - yoffset;

        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z=-1).
        let pixel = self.transform_inverse * Point::new(worldx, worldy, -1.0);
        let origin = self.transform_inverse * Point::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: WorldImpl) -> Canvas {
        let mut image = Canvas::new(self.width, self.height);

        let points: Vec<(i32, i32)> = (0..self.height)
            .into_iter()
            .map(|row| (0..self.width).into_iter().map(move |col| (row, col)))
            .flatten()
            .collect();

        let colors: Vec<(i32, i32, Color)> = points
            .par_iter()
            .map(|&(row, col)| {
                let ray = self.ray_for_pixel(col, row);
                (row, col, world.color_at(ray))
            })
            .collect();

        colors
            .iter()
            .for_each(|&(row, col, color)| image.write_pixel(col, row, color));

        image
    }
}
