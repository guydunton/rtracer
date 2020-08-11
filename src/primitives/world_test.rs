use super::{
    super::{Material, PointLight, Sphere, World},
    Ray,
};
use crate::{Color, Matrix4x4, Point, Vector};

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

    let sphere1 = Sphere::new(Matrix4x4::identity(), material);
    let sphere2 = Sphere::new(Matrix4x4::scaling(0.5, 0.5, 0.5), Material::default());

    let world = World::generate_default();

    assert_eq!(world.lights(), vec![light]);
    assert_eq!(world.objects(), vec![sphere1, sphere2]);
}

#[test]
fn intersect_a_world_with_a_ray() {
    let w = World::generate_default();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let xs = w.ray_intersects(r);

    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t(), 4.0);
    assert_eq!(xs[1].t(), 4.5);
    assert_eq!(xs[2].t(), 5.5);
    assert_eq!(xs[3].t(), 6.0);
}
