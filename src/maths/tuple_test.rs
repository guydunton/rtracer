
use super::point::Point;
use super::vector::Vector;
use super::Tuple;

#[test]
fn can_create_a_tuple_and_get_its_values() {
    let t = Tuple::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(t.x(), 1.0);
    assert_eq!(t.y(), 2.0);
    assert_eq!(t.z(), 3.0);
    assert_eq!(t.w(), 4.0);
}

#[test]
fn tuple_with_w_1_is_a_vector() {
    let t = Tuple::new(1.0, 2.0, 3.0, 0.0);
    assert_eq!(t.is_vector(), true);
    assert_eq!(t.is_point(), false);
}

#[test]
fn tuple_with_w_0_is_a_point() {
    let t = Tuple::new(1.0, 2.0, 3.0, 1.0);
    assert_eq!(t.is_point(), true);
    assert_eq!(t.is_vector(), false);
}

#[test]
fn point_creates_a_tuple_with_w_1() {
    let p = Point::new(1.0, 2.0, 3.0);
    assert_eq!(p.w(), 1.0);
}

#[test]
fn vector_create_a_tuple_with_w_0() {
    let v = Vector::new(1.0, 2.0, 3.0);
    assert_eq!(v.w(), 0.0);
}

#[test]
fn tuples_compare_correctly() {
    let a = Tuple::new(1.0, 2.0, 3.0, 0.0);
    let b = Tuple::new(1.0, 2.0, 3.0, 0.0);

    assert_eq!(a, b);
}

#[test]
fn adding_tuples_works() {
    let a = Tuple::new(3.0, -2.0, 5.0, 1.0);
    let b = Tuple::new(-2.0, 3.0, 1.0, 0.0);

    assert_eq!(a + b, Tuple::new(1.0, 1.0, 6.0, 1.0));
}

#[test]
fn subtracting_2_points() {
    let p1 = Point::new(3.0, 2.0, 1.0);
    let p2 = Point::new(5.0, 6.0, 7.0);

    assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
}

#[test]
fn subtracting_2_actual_points() {
    let p1 = Point::new(3.0, 2.0, 1.0);
    let p2 = Point::new(5.0, 6.0, 7.0);

    assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
}

#[test]
fn subtracting_a_vector_from_a_point() {
    let p = Tuple::point(3.0, 2.0, 1.0);
    let v = Tuple::vector(5.0, 6.0, 7.0);

    assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
}

#[test]
fn subtracting_a_real_vector_from_a_real_point() {
    let p = Point::new(3.0, 2.0, 1.0);
    let v = Vector::new(5.0, 6.0, 7.0);

    assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0));
}

#[test]
fn subtracting_2_vectors() {
    let v1 = Tuple::vector(3.0, 2.0, 1.0);
    let v2 = Tuple::vector(5.0, 6.0, 7.0);

    assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
}

#[test]
fn subtracting_2_actual_vectors() {
    let v1 = Vector::new(3.0, 2.0, 1.0);
    let v2 = Vector::new(5.0, 6.0, 7.0);

    assert_eq!(v1 - v2, Vector::new(-2.0, -4.0, -6.0));
}

#[test]
fn tuples_can_be_negated() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
}

#[test]
fn tuple_can_be_multiplied_by_a_scaler() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
}

#[test]
fn tuple_can_be_divided_by_a_scaler() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn compute_the_length_of_a_vector() {
    let v = Vector::new(1.0, 0.0, 0.0);
    assert_eq!(v.len(), 1.0);
}

#[test]
fn compute_the_length_of_a_different_vector() {
    let v = Vector::new(1.0, 2.0, 3.0);
    assert_eq!(v.len(), f64::sqrt(14.0));
}

#[test]
fn normalize_vector_4_0_0_gives_1_0_0() {
    let v = Vector::new(4.0, 0.0, 0.0);
    assert_eq!(v.normalize(), Vector::new(1.0, 0.0, 0.0));
}

#[test]
fn compute_the_dot_product_of_2_vectors() {
    let v1 = Vector::new(1.0, 2.0, 3.0);
    let v2 = Vector::new(2.0, 3.0, 4.0);
    assert_eq!(Vector::dot(v1, v2), 20.0);
}

#[test]
fn compute_the_cross_product_of_2_vectors() {
    let v1 = Vector::new(1.0, 2.0, 3.0);
    let v2 = Vector::new(2.0, 3.0, 4.0);
    assert_eq!(Vector::cross(v1, v2), Vector::new(-1.0, 2.0, -1.0));
    assert_eq!(Vector::cross(v2, v1), Vector::new(1.0, -2.0, 1.0));
}

#[test]
fn reflecting_a_vector_approaching_at_45() {
    let v = Vector::new(1.0, -1.0, 0.0);
    let n = Vector::new(0.0, 1.0, 0.0);

    assert_eq!(v.reflect(n), Vector::new(1.0, 1.0, 0.0));
}

#[test]
fn reflecting_a_vector_off_a_slanted_surface() {
    let v = Vector::new(0.0, -1.0, 0.0);
    let fraq = 2f64.sqrt() / 2.0;
    let n = Vector::new(fraq, fraq, 0.0);

    let r = v.reflect(n);
    assert_eq!(r, Vector::new(1.0, 0.0, 0.0));
}

#[test]
fn can_negate_a_vector() {
    let v = Vector::new(1.0, -2.0, 3.0);

    assert_eq!(-v, Vector::new(-1.0, 2.0, -3.0));
}
