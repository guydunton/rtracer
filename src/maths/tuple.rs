use super::float_compare::is_same;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    _x: f64,
    _y: f64,
    _z: f64,
    _w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple {
            _x: x,
            _y: y,
            _z: z,
            _w: w,
        }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            _x: x,
            _y: y,
            _z: z,
            _w: 1.0,
        }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            _x: x,
            _y: y,
            _z: z,
            _w: 0.0,
        }
    }

    pub fn dot(lhs: Tuple, rhs: Tuple) -> f64 {
        lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z() + lhs.w() * rhs.w()
    }

    pub fn cross(lhs: Tuple, rhs: Tuple) -> Tuple {
        Tuple::vector(
            lhs.y() * rhs.z() - lhs.z() * rhs.y(),
            lhs.z() * rhs.x() - lhs.x() * rhs.z(),
            lhs.x() * rhs.y() - lhs.y() * rhs.x(),
        )
    }

    pub fn x(&self) -> f64 {
        self._x
    }

    pub fn y(&self) -> f64 {
        self._y
    }

    pub fn z(&self) -> f64 {
        self._z
    }

    pub fn w(&self) -> f64 {
        self._w
    }

    pub fn is_vector(&self) -> bool {
        is_same(self.w(), 0.0)
    }

    pub fn is_point(&self) -> bool {
        is_same(self.w(), 1.0)
    }

    pub fn len(&self) -> f64 {
        f64::sqrt(
            self.x() * self.x() + self.y() * self.y() + self.z() * self.z() + self.w() * self.w(),
        )
    }

    pub fn normalize(&self) -> Tuple {
        let len = self.len();
        *self / len
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        is_same(self.x(), other.x())
            && is_same(self.y(), other.y())
            && is_same(self.z(), other.z())
            && is_same(self.w(), other.w())
    }
}

impl Add for Tuple {
    type Output = Self;
    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            _x: self.x() + other.x(),
            _y: self.y() + other.y(),
            _z: self.z() + other.z(),
            _w: self.w() + other.w(),
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            _x: self.x() - other.x(),
            _y: self.y() - other.y(),
            _z: self.z() - other.z(),
            _w: self.w() - other.w(),
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Tuple {
        Tuple {
            _x: -self.x(),
            _y: -self.y(),
            _z: -self.z(),
            _w: -self.w(),
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, rhs: f64) -> Tuple {
        Tuple {
            _x: self.x() * rhs,
            _y: self.y() * rhs,
            _z: self.z() * rhs,
            _w: self.w() * rhs,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;
    fn div(self, rhs: f64) -> Tuple {
        Tuple {
            _x: self.x() / rhs,
            _y: self.y() / rhs,
            _z: self.z() / rhs,
            _w: self.w() / rhs,
        }
    }
}