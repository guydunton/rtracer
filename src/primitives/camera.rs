use super::{world::WorldImpl, Ray, World};
use crate::{
    canvas::{Canvas, Color},
    maths::{is_same, round, Matrix4x4, Point, Vector},
};
use std::f64::consts::FRAC_PI_2;

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

#[test]
fn constructing_a_camera() {
    let width = 160;
    let height = 120;
    let fov = FRAC_PI_2;
    let camera = Camera::new(width, height, fov, Matrix4x4::identity());

    assert_eq!(camera.width(), width);
    assert_eq!(camera.height(), height);
    assert_eq!(camera.fov(), fov);
    assert_eq!(camera.transform(), Matrix4x4::identity());
}

#[test]
fn the_pixel_size_for_a_canvas() {
    let c1 = Camera::new(200, 125, FRAC_PI_2, Matrix4x4::identity());
    assert!(is_same(c1.pixel_size(), 0.01));

    let c2 = Camera::new(125, 200, FRAC_PI_2, Matrix4x4::identity());
    assert!(is_same(c2.pixel_size(), 0.01));
}

#[test]
fn constructing_a_ray_through_the_center_of_the_canvas() {
    let c = Camera::new(201, 101, FRAC_PI_2, Matrix4x4::identity());
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin(), Point::new(0.0, 0.0, 0.0));
    assert_eq!(r.direction(), Vector::new(0.0, 0.0, -1.0));
}

#[test]
fn constructing_a_ray_through_a_corner_of_the_canvas() {
    let c = Camera::new(201, 101, FRAC_PI_2, Matrix4x4::identity());
    let r = c.ray_for_pixel(0, 0);

    let direction = r.direction();
    assert_eq!(r.origin(), Point::new(0.0, 0.0, 0.0));
    assert_eq!(round(direction.x()), 0.66519);
    assert_eq!(round(direction.y()), 0.33259);
    assert_eq!(round(direction.z()), -0.66851);
}

#[test]
fn constructing_a_ray_when_the_camera_is_transformed() {
    let camera_translation =
        Matrix4x4::rotation_y(std::f64::consts::FRAC_PI_4).translate(0.0, -2.0, 5.0);
    let c = Camera::new(201, 101, FRAC_PI_2, camera_translation);
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin(), Point::new(0.0, 2.0, -5.0));
    assert_eq!(
        r.direction(),
        Vector::new(2f64.sqrt() / 2.0, 0.0, -2f64.sqrt() / 2.0)
    );
}

#[test]
fn rendering_a_world_with_a_camera() {
    let w = World::default().generate();
    let view_transform = Matrix4x4::view(
        Point::new(0.0, 0.0, -5.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::up(),
    );
    let c = Camera::new(11, 11, FRAC_PI_2, view_transform);

    let image = c.render(w);
    let color = image.pixel_at(5, 5);

    assert_eq!(round(color.r()), 0.38066);
    assert_eq!(round(color.g()), 0.47583);
    assert_eq!(round(color.b()), 0.28550);
}
