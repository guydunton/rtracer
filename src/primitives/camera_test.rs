use super::{Camera, World};
use crate::maths::{is_same, round, Matrix4x4, Point, Vector};
use std::f64::consts::FRAC_PI_2;

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
