mod canvas;
mod maths;
mod primitives;

use canvas::{save_canvas, Canvas, Color};
use maths::{Matrix4x4, Point, Vector};
use primitives::{Intersection, Material, PointLight, Ray, Sphere, World};

use rayon::prelude::*;

struct ColorResult {
    pos: (i32, i32),
    col: Color,
}

fn calculate_color(
    (row_addr, col_addr): &(i32, i32),
    sphere: Sphere,
    canvas_size: (i32, i32),
    camera_size: (f64, f64),
    light: PointLight,
) -> ColorResult {
    let row = *row_addr;
    let col = *col_addr;
    let ray_origin = Point::new(
        (col as f64 / canvas_size.0 as f64 * camera_size.0) - (camera_size.0 / 2.0),
        (row as f64 / canvas_size.1 as f64 * camera_size.1) - (camera_size.1 / 2.0),
        0.0,
    );

    // Create a ray
    let ray = Ray::new(ray_origin, Vector::new(0.0, 0.0, 1.0).normalize());

    // Check intersects with sphere
    let intersects = ray.intersects(sphere);

    // If so set the pixel color
    let color = match Intersection::hit(intersects) {
        Some(hit) => {
            let point = ray.position(hit.t());
            let normal = hit.object().normal_at(point);
            let eye = -ray.direction();
            hit.object().material().lighting(light, point, eye, normal)
        }
        _ => Color::black(),
    };

    ColorResult {
        pos: (row, col),
        col: color,
    }
}

fn main() {
    let mut canvas = Canvas::new(256, 256);

    let t = Matrix4x4::translation(0.0, 0.0, 5.0).scale(4.0, 4.0, 4.0);

    let mut material = Material::default();
    material.color = Color::new(0.2, 0.5, 1.0);
    let shape = Sphere::new(t, material);

    let light = PointLight::new(Point::new(-20.0, -30.0, -10.0), Color::white());

    // Find the list of points on the canvas
    let points: Vec<(i32, i32)> = (0..256)
        .into_iter()
        .map(|row| (0..256).into_iter().map(move |col| (row, col)))
        .flatten()
        .collect();

    // for each point calculate the ray
    let colors: Vec<ColorResult> = points
        .par_iter()
        .map(|point| calculate_color(point, shape, (256, 256), (10.0, 10.0), light))
        .collect();

    colors.iter().for_each(|color: &ColorResult| {
        canvas.write_pixel(color.pos.1, color.pos.0, color.col);
    });

    save_canvas(canvas, "out.png".to_owned()).unwrap();
}
