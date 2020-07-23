use super::{vector::Vector, Tuple};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    data: Tuple,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            data: Tuple::point(x, y, z),
        }
    }

    pub fn new_from_tuple(data: Tuple) -> Self {
        Self { data }
    }

    pub fn data(&self) -> Tuple {
        self.data
    }

    pub fn x(&self) -> f64 {
        self.data.x()
    }

    pub fn y(&self) -> f64 {
        self.data.y()
    }

    pub fn z(&self) -> f64 {
        self.data.z()
    }

    pub fn w(&self) -> f64 {
        self.data.w()
    }
}

impl Sub for Point {
    type Output = Vector;
    fn sub(self, rhs: Point) -> Vector {
        Vector::new_from_tuple(self.data - rhs.data)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, rhs: Vector) -> Point {
        Point::new_from_tuple(self.data - rhs.data())
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Point {
        Point::new_from_tuple(self.data + rhs.data())
    }
}
