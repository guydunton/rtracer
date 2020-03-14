use super::matrix_methods::MatrixMethods;
use std::cmp::PartialEq;

#[derive(Debug, Clone, Copy)]
pub struct Matrix3x3 {
    data: [f64; 9],
}

impl Matrix3x3 {
    pub fn new(
        m00: f64,
        m01: f64,
        m02: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m20: f64,
        m21: f64,
        m22: f64,
    ) -> Matrix3x3 {
        Matrix3x3 {
            data: [m00, m01, m02, m10, m11, m12, m20, m21, m22],
        }
    }

    pub fn at(&self, row: usize, col: usize) -> f64 {
        self.methods().at(row, col)
    }

    fn methods(&self) -> MatrixMethods {
        MatrixMethods::new(&self.data, 3)
    }
}

impl PartialEq for Matrix3x3 {
    fn eq(&self, rhs: &Matrix3x3) -> bool {
        self.methods() == rhs.methods()
    }
}
