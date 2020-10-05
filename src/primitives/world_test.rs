use super::{Intersection, Material, PointLight, Ray, Shape, World};
use crate::{image::Color, Matrix4x4, Point, Vector};

#[test]
fn creating_a_world() {
    let world = World::new().generate();

    assert_eq!(world.lights().len(), 0);
    assert_eq!(world.objects().len(), 0);
}

#[test]
fn the_default_world() {
    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white());

    let mut material = Material::default();
    material.color = Color::new(0.8, 1.0, 0.6);
    material.diffuse = 0.7;
    material.specular = 0.2;

    let sphere1 = Shape::sphere(Matrix4x4::identity(), material);
    let sphere2 = Shape::sphere(Matrix4x4::scaling(0.5, 0.5, 0.5), Material::default());

    let world = World::default().generate();

    assert_eq!(world.lights(), vec![light]);
    assert_eq!(world.objects(), vec![sphere1, sphere2]);
}

#[test]
fn intersect_a_world_with_a_ray() {
    let w = World::default().generate();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let xs = w.ray_intersects(r);

    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t(), 4.0);
    assert_eq!(xs[1].t(), 4.5);
    assert_eq!(xs[2].t(), 5.5);
    assert_eq!(xs[3].t(), 6.0);
}

#[allow(unused)]
fn round(v: f64) -> f64 {
    const SIG_FIGS: f64 = 100000.0;
    (v * SIG_FIGS).round() / SIG_FIGS
}

#[test]
fn shading_an_intersection() {
    let w = World::default().generate();
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape = w.objects()[0];
    let i = Intersection::new(4.0, shape);
    let comps = i.prepare_computations(ray);

    let c = w.shade_hit(comps);

    assert_eq!(round(c.r()), 0.38066);
    assert_eq!(round(c.g()), 0.47583);
    assert_eq!(round(c.b()), 0.28550);
}

#[test]
fn shading_an_intersection_from_the_inside() {
    let w = World::default()
        .reset_lights()
        .add_light(PointLight::new(Point::new(0.0, 0.25, 0.0), Color::white()))
        .generate();

    let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let shape = w.objects()[1];

    let i = Intersection::new(0.5, shape);
    let comps = i.prepare_computations(ray);
    let c = w.shade_hit(comps);

    assert_eq!(round(c.r()), 0.90498);
    assert_eq!(round(c.g()), 0.90498);
    assert_eq!(round(c.b()), 0.90498);
}

#[test]
fn the_color_when_a_ray_misses() {
    let w = World::default().generate();
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let c = w.color_at(ray);
    assert_eq!(round(c.r()), 0.38066);
    assert_eq!(round(c.g()), 0.47583);
    assert_eq!(round(c.b()), 0.28550);
}

#[test]
fn the_color_when_a_ray_hits() {
    let w = World::default().generate();
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let c = w.color_at(ray);

    assert_eq!(round(c.r()), 0.38066);
    assert_eq!(round(c.g()), 0.47583);
    assert_eq!(round(c.b()), 0.28550);
}

#[test]
fn the_color_with_an_intersection_behind_the_ray() {
    let mut material1 = Material::default();
    material1.color = Color::new(0.8, 1.0, 0.6);
    material1.diffuse = 0.7;
    material1.specular = 0.2;
    material1.ambient = 1.0;

    let mut material2 = Material::default();
    material2.ambient = 1.0;

    let sphere1 = Shape::sphere(Matrix4x4::identity(), material1);
    let sphere2 = Shape::sphere(Matrix4x4::scaling(0.5, 0.5, 0.5), material2);

    let w = World::new()
        .add_light(PointLight::new(
            Point::new(-10.0, 10.0, -10.0),
            Color::white(),
        ))
        .add_object(sphere1)
        .add_object(sphere2)
        .generate();

    let ray = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
    let c = w.color_at(ray);

    assert_eq!(c, material2.color);
}

#[test]
fn testing_is_shadowed_at_various_points() {
    let w = World::default().generate();
    assert_eq!(w.is_shadowed(Point::new(0.0, 10.0, 0.0)), false);
    assert_eq!(w.is_shadowed(Point::new(10.0, -10.0, 10.0)), true);
    assert_eq!(w.is_shadowed(Point::new(-20.0, 20.0, -20.0)), false);
    assert_eq!(w.is_shadowed(Point::new(-2.0, 2.0, -2.0)), false);
}

#[test]
fn shade_hit_is_given_an_intersection_in_shadow() {
    let s2 = Shape::sphere_from_transformation(Matrix4x4::translation(0.0, 0.0, 10.0));
    let w = World::new()
        .add_light(PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white()))
        .add_object(Shape::default())
        .add_object(s2)
        .generate();

    let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
    let i = Intersection::new(4.0, s2);
    let comps = i.prepare_computations(ray);

    let c = w.shade_hit(comps);
    assert_eq!(c, Color::new(0.1, 0.1, 0.1));
}
