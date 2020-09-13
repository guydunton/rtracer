use super::Shape;
use crate::{Material, Matrix4x4, Point, Vector};
use std::f64::consts::PI;

fn round(v: f64) -> f64 {
    const SIG_FIGS: f64 = 100000.0;
    (v * SIG_FIGS).round() / SIG_FIGS
}

#[test]
fn the_normal_on_different_axis() {
    let s = Shape::default();
    let n1 = s.normal_at(Point::new(1.0, 0.0, 0.0));
    assert_eq!(n1, Vector::new(1.0, 0.0, 0.0));
    let n2 = s.normal_at(Point::new(0.0, 1.0, 0.0));
    assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
    let n3 = s.normal_at(Point::new(0.0, 0.0, 1.0));
    assert_eq!(n3, Vector::new(0.0, 0.0, 1.0));

    let p = 3f64.sqrt() / 3.0;
    let n4 = s.normal_at(Point::new(p, p, p));
    assert_eq!(n4, Vector::new(p, p, p));
}

#[test]
fn the_normal_is_a_normalized_vector() {
    let s = Shape::default();
    let p = 3f64.sqrt() / 3.0;
    let n = s.normal_at(Point::new(p, p, p));
    assert_eq!(n, n.normalize());
}

#[test]
fn computing_the_normal_on_a_translated_sphere() {
    let s = Shape::sphere_from_transformation(Matrix4x4::translation(0.0, 1.0, 0.0));
    let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));

    assert_eq!(round(n.x()), 0.0);
    assert_eq!(round(n.y()), 0.70711);
    assert_eq!(round(n.z()), -0.70711);
    assert_eq!(round(n.w()), 0.0);
}

#[test]
fn computing_the_normal_on_a_transformed_sphere() {
    //let m = Matrix4x4::scaling(1.0, 0.5, 1.0).rotate_z(PI / 5.0);
    let m = Matrix4x4::scaling(1.0, 0.5, 1.0).rotate_z(PI / 5.0);
    let s = Shape::sphere_from_transformation(m);

    let p = 2f64.sqrt() / 2.0;
    let n = s.normal_at(Point::new(0.0, p, -p));
    assert_eq!(round(n.x()), 0.0);
    assert_eq!(round(n.y()), 0.97014);
    assert_eq!(round(n.z()), -0.24254);
    assert_eq!(round(n.w()), 0.0);
}

#[test]
fn a_sphere_has_a_default_material() {
    let s = Shape::default();
    assert_eq!(s.material(), Material::default());
}

#[test]
fn a_sphere_may_be_assigned_a_material() {
    let mut m = Material::default();
    m.ambient = 1.0;
    let s = Shape::sphere(Matrix4x4::identity(), m);
    assert_eq!(s.material(), m);
}

#[test]
fn the_normal_of_a_plane_is_constant_everywhere() {
    let p = Shape::plane_default();

    let n1 = p.normal_at(Point::new(0.0, 0.0, 0.0));
    let n2 = p.normal_at(Point::new(10.0, 0.0, -10.0));
    let n3 = p.normal_at(Point::new(-5.0, 0.0, 150.0));

    assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
    assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
    assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
}
