
use super::{Intersection, Ray, Sphere};
use crate::{Matrix4x4, Point, Vector};

#[test]
fn creating_and_querying_a_ray() {
    let origin = Point::new(1.0, 2.0, 3.0);
    let direction = Vector::new(4.0, 5.0, 6.0);

    let ray = Ray::new(origin, direction);

    assert_eq!(ray.origin(), origin);
    assert_eq!(ray.direction(), direction);
}

#[test]
fn computing_a_point_from_a_distance() {
    let ray = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

    assert_eq!(ray.position(0.0), Point::new(2.0, 3.0, 4.0));
    assert_eq!(ray.position(1.0), Point::new(3.0, 3.0, 4.0));
    assert_eq!(ray.position(-1.0), Point::new(1.0, 3.0, 4.0));
    assert_eq!(ray.position(2.5), Point::new(4.5, 3.0, 4.0));
}

#[test]
fn a_ray_intersects_a_sphere_at_2_locations() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Sphere::default();

    let intersections = r.intersects(s);
    assert_eq!(intersections[0].t(), 4.0);
    assert_eq!(intersections[1].t(), 6.0);
}

#[test]
fn a_ray_intersects_a_sphere_at_a_tangent() {
    let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Sphere::default();
    let i = r.intersects(s);

    assert_eq!(i[0].t(), 5.0);
    assert_eq!(i[1].t(), 5.0);
}

#[test]
fn a_ray_misses_a_sphere() {
    let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Sphere::default();

    let i = r.intersects(s);

    assert_eq!(i.len(), 0);
}

#[test]
fn a_ray_originates_inside_a_sphere() {
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let s = Sphere::default();

    let i = r.intersects(s);
    assert_eq!(i[0].t(), -1.0);
    assert_eq!(i[1].t(), 1.0);
}

#[test]
fn a_sphere_is_behind_a_ray() {
    let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Sphere::default();

    let i = r.intersects(s);
    assert_eq!(i[0].t(), -6.0);
    assert_eq!(i[1].t(), -4.0);
    assert_eq!(i[0].object(), s);
    assert_eq!(i[1].object(), s);
}

#[test]
fn an_intersection_encapsulates_t_and_object() {
    let s = Sphere::default();
    let i = Intersection::new(3.5, s);

    assert_eq!(i.t(), 3.5);
    assert_eq!(i.object(), s);
}

#[test]
fn the_hit_when_all_intersections_have_positive_t() {
    let s = Sphere::default();
    let i1 = Intersection::new(1.0, s);
    let i2 = Intersection::new(2.0, s);

    let intersections = vec![i1, i2];
    let hit = Intersection::hit(intersections).unwrap();

    assert_eq!(hit, i1);
}

#[test]
fn the_hit_when_some_intersections_have_negative_t() {
    let s = Sphere::default();
    let i1 = Intersection::new(-1.0, s);
    let i2 = Intersection::new(1.0, s);

    let intersections = vec![i1, i2];
    let hit = Intersection::hit(intersections).unwrap();

    assert_eq!(hit, i2);
}

#[test]
fn the_hit_when_all_intersections_have_negative_t() {
    let s = Sphere::default();
    let i1 = Intersection::new(-2.0, s);
    let i2 = Intersection::new(-1.0, s);

    let intersections = vec![i1, i2];
    let hit = Intersection::hit(intersections);

    assert_eq!(hit, None);
}

#[test]
fn the_hit_is_always_the_lowest_nonnegative_intersection() {
    let s = Sphere::default();
    let i1 = Intersection::new(5.0, s);
    let i2 = Intersection::new(7.0, s);
    let i3 = Intersection::new(-3.0, s);
    let i4 = Intersection::new(2.0, s);
    let hit = Intersection::hit(vec![i1, i2, i3, i4]).unwrap();

    assert_eq!(hit, i4);
}

#[test]
fn translate_a_ray() {
    let r1 = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
    let m = Matrix4x4::translation(3.0, 4.0, 5.0);
    let r2 = r1.transform(m);

    assert_eq!(r2.origin(), Point::new(4.0, 6.0, 8.0));
    assert_eq!(r2.direction(), Vector::new(0.0, 1.0, 0.0));
}

#[test]
fn scale_a_ray() {
    let r1 = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
    let m = Matrix4x4::scaling(2.0, 3.0, 4.0);
    let r2 = r1.transform(m);

    assert_eq!(r2.origin(), Point::new(2.0, 6.0, 12.0));
    assert_eq!(r2.direction(), Vector::new(0.0, 3.0, 0.0));
}

#[test]
fn a_spheres_default_transformation() {
    let s = Sphere::default();
    assert_eq!(s.transformation(), Matrix4x4::identity());
}

#[test]
fn set_a_spheres_transformation() {
    let t = Matrix4x4::translation(2.0, 3.0, 4.0);
    let s2 = Sphere::sphere_from_transformation(t);

    assert_eq!(s2.transformation(), t);
}

#[test]
fn intersecting_a_scaled_sphere_with_a_ray() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Sphere::sphere_from_transformation(Matrix4x4::scaling(2.0, 2.0, 2.0));

    let intersections = r.intersects(s);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].t(), 3.0);
    assert_eq!(intersections[1].t(), 7.0);
}

#[test]
fn intersecting_a_translated_sphere_with_a_ray() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Sphere::sphere_from_transformation(Matrix4x4::translation(5.0, 0.0, 0.0));

    let intersections = r.intersects(s);

    assert_eq!(intersections.len(), 0);
}

#[test]
fn precomputing_the_state_of_an_intersection() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape = Sphere::default();
    let i = Intersection::new(4.0, shape);

    let comps = i.prepare_computations(r);
    assert_eq!(comps.t(), i.t());
    assert_eq!(comps.object(), i.object());
    assert_eq!(comps.point(), Point::new(0.0, 0.0, -1.0));
    assert_eq!(comps.eyev(), Vector::new(0.0, 0.0, -1.0));
    assert_eq!(comps.normalv(), Vector::new(0.0, 0.0, -1.0));
}

#[test]
fn the_hit_when_an_intersection_occurs_on_the_outside() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape = Sphere::default();
    let i = Intersection::new(4.0, shape);

    let comps = i.prepare_computations(ray);
    assert_eq!(comps.inside(), false);
}

#[test]
fn the_hit_when_an_intersection_occurs_on_the_inside() {
    let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let shape = Sphere::default();
    let i = Intersection::new(1.0, shape);

    let comps = i.prepare_computations(ray);
    assert_eq!(comps.point(), Point::new(0.0, 0.0, 1.0));
    assert_eq!(comps.eyev(), Vector::new(0.0, 0.0, -1.0));
    assert_eq!(comps.inside(), true);
    assert_eq!(comps.normalv(), Vector::new(0.0, 0.0, -1.0));
}

#[test]
fn the_hit_should_offset_the_point() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape = Sphere::sphere_from_transformation(Matrix4x4::translation(0.0, 0.0, 1.0));

    let i = Intersection::new(5.0, shape);
    let comps = i.prepare_computations(ray);

    assert!(comps.over_point().z() < (-std::f32::EPSILON as f64 / 2.0));
    assert!(comps.point().z() > comps.over_point().z());
}
