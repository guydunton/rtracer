#![allow(clippy::too_many_arguments)]
use super::matrix_methods::MatrixMethods;
use super::{Matrix3x3, Point, Tuple, Vector};
use std::cmp::PartialEq;
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4 {
    data: [f64; 16],
}

impl Matrix4x4 {
    pub fn new(
        m00: f64,
        m01: f64,
        m02: f64,
        m03: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m13: f64,
        m20: f64,
        m21: f64,
        m22: f64,
        m23: f64,
        m30: f64,
        m31: f64,
        m32: f64,
        m33: f64,
    ) -> Matrix4x4 {
        Matrix4x4 {
            data: [
                m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23, m30, m31, m32, m33,
            ],
        }
    }

    fn new_empty() -> Matrix4x4 {
        Matrix4x4 { data: [0.0; 16] }
    }

    pub fn identity() -> Matrix4x4 {
        Matrix4x4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix4x4 {
        let mut identity = Self::identity();
        identity.set_at(0, 3, x);
        identity.set_at(1, 3, y);
        identity.set_at(2, 3, z);
        identity
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4x4 {
        Self::new(
            x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn rotation_x(radians: f64) -> Matrix4x4 {
        Self::new(
            // Row 1
            1.0,
            0.0,
            0.0,
            0.0,
            // Row 2
            0.0,
            radians.cos(),
            -radians.sin(),
            0.0,
            // Row 3
            0.0,
            radians.sin(),
            radians.cos(),
            0.0,
            // Row 4
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    pub fn rotation_y(radians: f64) -> Matrix4x4 {
        Self::new(
            // Row 1
            radians.cos(),
            0.0,
            radians.sin(),
            0.0,
            // Row 2
            0.0,
            1.0,
            0.0,
            0.0,
            // Row 3
            -radians.sin(),
            0.0,
            radians.cos(),
            0.0,
            // Row 4
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    pub fn rotation_z(radians: f64) -> Matrix4x4 {
        Self::new(
            // Row 1
            radians.cos(),
            -radians.sin(),
            0.0,
            0.0,
            // Row 2
            radians.sin(),
            radians.cos(),
            0.0,
            0.0,
            // Row 3
            0.0,
            0.0,
            1.0,
            0.0,
            // Row 4
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4x4 {
        Self::new(
            // Row 1
            1.0, xy, xz, 0.0, // Row 2
            yx, 1.0, yz, 0.0, // Row 3
            zx, zy, 1.0, 0.0, // Row 4
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn view(from: Point, to: Point, up: Vector) -> Self {
        let forward = (to - from).normalize();
        let upn = up.normalize();
        let left = Vector::cross(forward, upn);
        let true_up = Vector::cross(left, forward);

        let orientation = Matrix4x4::new(
            left.x(),
            left.y(),
            left.z(),
            0.0, // row 0
            true_up.x(),
            true_up.y(),
            true_up.z(),
            0.0, // row 1
            -forward.x(),
            -forward.y(),
            -forward.z(),
            0.0, // row 2
            0.0,
            0.0,
            0.0,
            1.0, // row 3
        );

        orientation * Self::translation(-from.x(), -from.y(), -from.z())
    }

    pub fn at(&self, row: usize, col: usize) -> f64 {
        self.methods().at(row, col)
    }

    fn methods(&self) -> MatrixMethods {
        MatrixMethods::new(&self.data, 4)
    }

    pub fn set_at(&mut self, row: usize, col: usize, val: f64) {
        self.data[row * 4 + col] = val;
    }

    pub fn transpose(&self) -> Matrix4x4 {
        let mut result = Matrix4x4::new_empty();

        for row in 0..4 {
            for col in 0..4 {
                result.set_at(
                    row,
                    col,
                    self.at(col, row), // reverse the row & column
                );
            }
        }

        result
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix3x3 {
        let data = self.methods().submatrix_data(row, col);
        Matrix3x3::new_from_slice(&data[..])
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        let sub = self.submatrix(row, col);
        sub.determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            minor * -1.0
        }
    }

    pub fn determinant(&self) -> f64 {
        (0..4).map(|i| self.at(0, i) * self.cofactor(0, i)).sum()
    }

    pub fn is_invertable(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Option<Matrix4x4> {
        if !self.is_invertable() {
            return None;
        }

        let mut result = Matrix4x4::new_empty();
        let determinant = self.determinant();

        for row in 0..4 {
            for col in 0..4 {
                let cofactor = self.cofactor(row, col);
                result.set_at(col, row, cofactor / determinant);
            }
        }

        Some(result)
    }

    pub fn rotate_x(&self, radians: f64) -> Matrix4x4 {
        Self::rotation_x(radians) * *self
    }

    pub fn rotate_y(&self, radians: f64) -> Matrix4x4 {
        Self::rotation_y(radians) * *self
    }

    pub fn rotate_z(&self, radians: f64) -> Matrix4x4 {
        Self::rotation_z(radians) * *self
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Matrix4x4 {
        Self::scaling(x, y, z) * *self
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Matrix4x4 {
        Self::translation(x, y, z) * *self
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, rhs: &Matrix4x4) -> bool {
        self.methods() == rhs.methods()
    }
}

impl Mul for Matrix4x4 {
    type Output = Self;

    #[allow(clippy::op_ref)]
    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        self * &rhs
    }
}

impl Mul<&Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: &Matrix4x4) -> Matrix4x4 {
        let mut output = Matrix4x4::new_empty();

        for row in 0..4 {
            for col in 0..4 {
                let val = self.at(row, 0) * rhs.at(0, col)
                    + self.at(row, 1) * rhs.at(1, col)
                    + self.at(row, 2) * rhs.at(2, col)
                    + self.at(row, 3) * rhs.at(3, col);
                output.set_at(row, col, val);
            }
        }

        output
    }
}

#[allow(clippy::suspicious_arithmetic_impl)] // Not sure I trust this one
impl Mul<Tuple> for Matrix4x4 {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Tuple {
        let val_at_row = |row| {
            self.at(row, 0) * rhs.x()
                + self.at(row, 1) * rhs.y()
                + self.at(row, 2) * rhs.z()
                + self.at(row, 3) * rhs.w()
        };

        Tuple::new(val_at_row(0), val_at_row(1), val_at_row(2), val_at_row(3))
    }
}

impl Mul<Vector> for Matrix4x4 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Vector {
        Vector::new_from_tuple(self * rhs.data())
    }
}

impl Mul<Point> for Matrix4x4 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point::new_from_tuple(self * rhs.data())
    }
}
