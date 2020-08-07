// This is a library to the code won't be used yet
#![allow(dead_code)]
use super::{Intersection, Sphere};
use crate::{
    maths::{Matrix4x4, Vector},
    Point,
};

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }

    pub fn position(&self, time: f64) -> Point {
        self.origin + self.direction * time
    }

    pub fn intersects(&self, shape: Sphere) -> Vec<Intersection> {
        let ray2 = self.transform(shape.transformation().inverse().unwrap());
        let sphere_to_ray = ray2.origin - Point::new(0.0, 0.0, 0.0);
        let a = Vector::dot(ray2.direction, ray2.direction);
        let b = 2.0 * Vector::dot(ray2.direction, sphere_to_ray);
        let c = Vector::dot(sphere_to_ray, sphere_to_ray) - 1.0;

        let discriminant = b * b - (4.0 * a * c);

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), shape);
        let t2 = Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), shape);
        vec![t1, t2]
    }

    pub fn transform(&self, m: Matrix4x4) -> Ray {
        Ray::new(m * self.origin, m * self.direction)
    }
}
