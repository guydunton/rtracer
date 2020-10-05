use crate::{image::Color, Point, Vector};

use super::{Material, PointLight};

fn round(v: f64) -> f64 {
    const SIG_FIGS: f64 = 100000.0;
    (v * SIG_FIGS).round() / SIG_FIGS
}

#[test]
fn lighting_with_the_eye_between_the_light_and_the_surface() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);

    let eyev = Vector::new(0.0, 0.0, -1.0);
    let normalv = Vector::new(0.0, 0.0, -1.0);

    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let result = m.lighting(&vec![light], position, eyev, normalv, false);
    assert_eq!(result, Color::new(1.9, 1.9, 1.9));
}

#[test]
fn lighting_with_the_eye_between_light_and_surface_eye_offset_45() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eyev = Vector::new(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
    let normalv = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let result = m.lighting(&vec![light], position, eyev, normalv, false);
    assert_eq!(result, Color::new(1.0, 1.0, 1.0));
}

#[test]
fn lighting_with_eye_opposite_surface_lighting_offset_45() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);

    let eyev = Vector::new(0.0, 0.0, -1.0);
    let normalv = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let result = m.lighting(&vec![light], position, eyev, normalv, false);
    assert_eq!(round(result.r()), 0.7364);
    assert_eq!(round(result.g()), 0.7364);
    assert_eq!(round(result.b()), 0.7364);
}

#[test]
fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);

    let eyev = Vector::new(0.0, -2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
    let normalv = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let result = m.lighting(&vec![light], position, eyev, normalv, false);
    assert_eq!(round(result.r()), 1.6364);
    assert_eq!(round(result.g()), 1.6364);
    assert_eq!(round(result.b()), 1.6364);
}

#[test]
fn lighting_with_the_light_behind_the_surface() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);

    let eyev = Vector::new(0.0, 0.0, -1.0);
    let normalv = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

    let result = m.lighting(&vec![light], position, eyev, normalv, false);
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}

#[test]
fn lighting_with_the_surface_in_shadow() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eyev = Vector::new(0.0, 0.0, -1.0);
    let normalv = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
    let in_shadow = true;

    let result = m.lighting(&vec![light], position, eyev, normalv, in_shadow);
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}
