use super::Sphere;
use crate::maths::{Point, Vector};

#[derive(Copy, Clone, Debug)]
pub struct IntersectionStats {
    t: f64,
    object: Sphere,
    point: Point,
    eyev: Vector,
    normalv: Vector,
    inside: bool,
}

impl IntersectionStats {
    pub fn new(
        t: f64,
        object: Sphere,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        inside: bool,
    ) -> Self {
        Self {
            t,
            object,
            point,
            eyev,
            normalv,
            inside,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn object(&self) -> Sphere {
        self.object
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn eyev(&self) -> Vector {
        self.eyev
    }

    pub fn normalv(&self) -> Vector {
        self.normalv
    }

    pub fn inside(&self) -> bool {
        self.inside
    }
}