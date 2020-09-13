mod image;
mod maths;
mod primitives;

use image::{save_canvas, Color};
use maths::{Matrix4x4, Point, Vector};
use primitives::{Camera, Material, PointLight, Shape, World};

fn main() {
    // Create floor
    let floor = Shape::plane_default();

    // Create middle
    let middle_transform = Matrix4x4::translation(-0.5, 1.0, 0.5);
    let mut middle_mat = Material::default();
    middle_mat.color = Color::new(0.1, 1.0, 0.5);
    middle_mat.diffuse = 0.7;
    middle_mat.specular = 0.3;
    let middle = Shape::sphere(middle_transform, middle_mat);

    // Create right
    let right_transform = Matrix4x4::translation(1.5, 0.5, -0.5).scale(0.5, 0.5, 0.5);
    let mut right_mat = Material::default();
    right_mat.color = Color::new(0.5, 1.0, 0.1);
    right_mat.diffuse = 0.7;
    right_mat.specular = 0.3;
    let right = Shape::sphere(right_transform, right_mat);

    // Create left
    let left_translation = Matrix4x4::translation(-1.5, 0.33, -0.75).scale(0.33, 0.33, 0.33);
    let mut left_mat = Material::default();
    left_mat.color = Color::new(1.0, 0.8, 0.1);
    left_mat.diffuse = 0.7;
    left_mat.specular = 0.3;
    let left = Shape::sphere(left_translation, left_mat);

    // Create world
    let world = World::new()
        .add_light(PointLight::new(
            Point::new(-10.0, 10.0, -10.0),
            Color::white(),
        ))
        .add_light(PointLight::new(
            Point::new(0.0, 5.0, 2.0),
            Color::new(0.7, 1.0, 0.8),
        ))
        .add_object(floor)
        .add_object(middle)
        .add_object(right)
        .add_object(left)
        .generate();

    // quality 1 == 128 * 64
    // quality 4 == 1024 * 512
    let quality_multiplier = 4;
    let (width, height) = (
        128 * 2i32.pow(quality_multiplier),
        64 * 2i32.pow(quality_multiplier),
    );

    let view_transform = Matrix4x4::view(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::up(),
    );
    let camera = Camera::new(width, height, std::f64::consts::FRAC_PI_3, view_transform);

    let canvas = camera.render(world);
    save_canvas(canvas, "out.png".to_owned()).unwrap();
}
