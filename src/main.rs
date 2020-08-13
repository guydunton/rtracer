mod canvas;
mod maths;
mod primitives;

use canvas::{save_canvas, Color};
use maths::{Matrix4x4, Point, Vector};
use primitives::{Camera, Material, PointLight, Sphere, World};

fn main() {
    // Create the floor
    let floor_transform = Matrix4x4::scaling(10.0, 0.01, 10.0);
    let mut floor_mat = Material::default();
    floor_mat.color = Color::new(0.8, 0.7, 0.7);
    floor_mat.specular = 0.0;
    let floor = Sphere::new(floor_transform, floor_mat);

    // Create the left wall
    let left_wall_transform = Matrix4x4::translation(0.0, 0.0, 5.0)
        .rotate_y(-std::f64::consts::FRAC_PI_4)
        .rotate_x(std::f64::consts::FRAC_PI_2)
        .scale(10.0, 0.01, 10.0);
    let left_wall = Sphere::new(left_wall_transform, floor_mat);

    // Create the right wall
    let right_wall_transform = Matrix4x4::translation(0.0, 0.0, 5.0)
        .rotate_y(std::f64::consts::FRAC_PI_4)
        .rotate_x(std::f64::consts::FRAC_PI_2)
        .scale(10.0, 0.01, 10.0);
    let right_wall = Sphere::new(right_wall_transform, floor_mat);

    // Create middle
    let middle_transform = Matrix4x4::translation(-0.5, 1.0, 0.5);
    let mut middle_mat = Material::default();
    middle_mat.color = Color::new(0.1, 1.0, 0.5);
    middle_mat.diffuse = 0.7;
    middle_mat.specular = 0.3;
    let middle = Sphere::new(middle_transform, middle_mat);

    // Create right
    let right_transform = Matrix4x4::translation(1.5, 0.5, -0.5).scale(0.5, 0.5, 0.5);
    let mut right_mat = Material::default();
    right_mat.color = Color::new(0.5, 1.0, 0.1);
    right_mat.diffuse = 0.7;
    right_mat.specular = 0.3;
    let right = Sphere::new(right_transform, right_mat);

    // Create left
    let left_translation = Matrix4x4::translation(-1.5, 0.33, -0.75).scale(0.33, 0.33, 0.33);
    let mut left_mat = Material::default();
    left_mat.color = Color::new(1.0, 0.8, 0.1);
    left_mat.diffuse = 0.7;
    left_mat.specular = 0.3;
    let left = Sphere::new(left_translation, left_mat);

    // Create world
    let world = World::new()
        .add_light(PointLight::new(
            Point::new(-10.0, 10.0, -10.0),
            Color::white(),
        ))
        // This second light is currently commented out because shadow calculations
        // cannot currently be done with multiple lights
        .add_light(PointLight::new(
            Point::new(0.0, 5.0, 2.0),
            Color::new(0.7, 1.0, 0.8),
        ))
        .add_object(floor)
        .add_object(left_wall)
        .add_object(right_wall)
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
