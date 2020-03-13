use super::matrix_methods::MatrixMethods;
use std::cmp::PartialEq;
use std::ops::Mul;

#[derive(Debug)]
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

    pub fn at(&self, row: usize, col: usize) -> f64 {
        self.methods().at(row, col)
    }

    fn methods(&self) -> MatrixMethods {
        MatrixMethods::new(&self.data, 4)
    }

    fn set_at(&mut self, row: usize, col: usize, val: f64) {
        self.data[row * 4 + col] = val;
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, rhs: &Matrix4x4) -> bool {
        self.methods() == rhs.methods()
    }
}

impl Mul for Matrix4x4 {
    type Output = Self;
    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        let mut output = Matrix4x4 { data: [0.0; 16] };

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
