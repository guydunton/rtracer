use super::matrix_methods::MatrixMethods;
use std::cmp::PartialEq;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy)]
pub struct Matrix2x2 {
    data: [f64; 4],
}

impl Matrix2x2 {
    pub fn new(m00: f64, m01: f64, m10: f64, m11: f64) -> Matrix2x2 {
        Matrix2x2 {
            data: [m00, m01, m10, m11],
        }
    }

    pub fn new_from_slice(data: &[f64]) -> Matrix2x2 {
        let new_data: [f64; 4] = data
            .try_into()
            .expect("Provided slice is wrong length for Matrix2x2");

        Matrix2x2 { data: new_data }
    }

    pub fn new_empty() -> Matrix2x2 {
        Matrix2x2 { data: [0.0; 4] }
    }

    pub fn at(&self, row: usize, col: usize) -> f64 {
        self.methods().at(row, col)
    }

    fn methods(&self) -> MatrixMethods {
        MatrixMethods::new(&self.data, 2)
    }

    pub fn determinant(&self) -> f64 {
        self.at(0, 0) * self.at(1, 1) - self.at(0, 1) * self.at(1, 0)
    }
}

impl PartialEq for Matrix2x2 {
    fn eq(&self, rhs: &Matrix2x2) -> bool {
        self.methods() == rhs.methods()
    }
}
