// This is a library to the code won't be used yet
#![allow(dead_code)]
use super::{shape::ShapeType, Intersection, Shape};
use crate::{
    maths::{Matrix4x4, Vector},
    Point,
};

#[derive(Copy, Clone)]
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

    pub fn intersects(&self, shape: Shape) -> Vec<Intersection> {
        let local_ray = self.transform(shape.transformation_inverse());

        // Maybe match on the shape type here
        match shape.shape_type() {
            ShapeType::Sphere => {
                let sphere_to_ray = local_ray.origin - Point::new(0.0, 0.0, 0.0);
                let a = Vector::dot(local_ray.direction, local_ray.direction);
                let b = 2.0 * Vector::dot(local_ray.direction, sphere_to_ray);
                let c = Vector::dot(sphere_to_ray, sphere_to_ray) - 1.0;

                let discriminant = b * b - (4.0 * a * c);

                if discriminant < 0.0 {
                    return vec![];
                }

                let t1 = Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), shape);
                let t2 = Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), shape);
                vec![t1, t2]
            }
            ShapeType::Plane => {
                if local_ray.direction.y().abs() < (f64::EPSILON * 100.0) {
                    return vec![];
                }
                let t = -local_ray.origin.y() / local_ray.direction.y();
                let t1 = Intersection::new(t, shape);
                vec![t1]
            }
        }
    }

    pub fn transform(&self, m: Matrix4x4) -> Ray {
        Ray::new(m * self.origin, m * self.direction)
    }
}
