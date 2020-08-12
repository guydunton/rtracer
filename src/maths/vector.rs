use super::Tuple;
use std::ops::{Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    data: Tuple,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {
            data: Tuple::vector(x, y, z),
        }
    }

    pub fn new_from_tuple(data: Tuple) -> Vector {
        Vector { data }
    }

    pub fn up() -> Self {
        Self::new(0.0, 1.0, 0.0)
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

    pub fn set_x(&mut self, x: f64) {
        self.data.set_x(x);
    }

    pub fn set_y(&mut self, y: f64) {
        self.data.set_y(y);
    }

    pub fn set_z(&mut self, z: f64) {
        self.data.set_z(z);
    }

    pub fn set_w(&mut self, w: f64) {
        self.data.set_w(w);
    }

    pub fn data(&self) -> Tuple {
        self.data
    }

    pub fn dot(lhs: Vector, rhs: Vector) -> f64 {
        lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z() + lhs.w() * rhs.w()
    }

    pub fn cross(lhs: Vector, rhs: Vector) -> Vector {
        Vector::new(
            lhs.y() * rhs.z() - lhs.z() * rhs.y(),
            lhs.z() * rhs.x() - lhs.x() * rhs.z(),
            lhs.x() * rhs.y() - lhs.y() * rhs.x(),
        )
    }

    pub fn len(&self) -> f64 {
        f64::sqrt(
            self.x() * self.x() + self.y() * self.y() + self.z() * self.z() + self.w() * self.w(),
        )
    }

    pub fn normalize(&self) -> Vector {
        let len = self.len();
        Vector::new_from_tuple(self.data() / len)
    }

    pub fn reflect(&self, normal: Vector) -> Vector {
        *self - normal * 2.0 * Vector::dot(*self, normal)
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        Vector::new_from_tuple(self.data - rhs.data)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Vector {
        Vector::new_from_tuple(self.data * rhs)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Vector::new_from_tuple(-self.data)
    }
}
