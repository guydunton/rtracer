use super::matrix_methods::MatrixMethods;
use super::Matrix2x2;
use std::cmp::PartialEq;
use std::convert::TryInto;

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

    pub fn new_from_slice(data: &[f64]) -> Matrix3x3 {
        let new_data: [f64; 9] = data
            .try_into()
            .expect("Provided slice is wrong length for Matrix2x2");

        Matrix3x3 { data: new_data }
    }

    pub fn at(&self, row: usize, col: usize) -> f64 {
        self.methods().at(row, col)
    }

    fn methods(&self) -> MatrixMethods {
        MatrixMethods::new(&self.data, 3)
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2x2 {
        let data = MatrixMethods::new(&self.data, 3).submatrix_data(row, col);
        Matrix2x2::new_from_slice(&data[..])
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let sub = self.submatrix(row, col);
        sub.determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if row + col % 2 == 0 {
            minor
        } else {
            minor * -1.0
        }
    }

    pub fn determinant(&self) -> f64 {
        (0..3).map(|i| self.at(0, i) * self.cofactor(0, i)).sum()
    }
}

impl PartialEq for Matrix3x3 {
    fn eq(&self, rhs: &Matrix3x3) -> bool {
        self.methods() == rhs.methods()
    }
}
