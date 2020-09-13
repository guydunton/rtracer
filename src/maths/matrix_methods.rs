use super::float_compare::is_same;
use std::cmp::PartialEq;

pub struct MatrixMethods<'a> {
    data: &'a [f64],
    size: usize,
}

impl<'a> MatrixMethods<'a> {
    pub fn new(data: &'a [f64], size: usize) -> MatrixMethods {
        MatrixMethods { data, size }
    }

    pub fn at(&self, row: usize, col: usize) -> f64 {
        if row > self.size || col > self.size {
            panic!("Row {} or col {} greater than size {}", row, col, self.size);
        }
        self.data[row * self.size + col]
    }

    pub fn submatrix_data(&self, row: usize, col: usize) -> Vec<f64> {
        let mut data = Vec::new();
        data.reserve(self.size * self.size);

        for current_row in 0..self.size {
            for current_col in 0..self.size {
                if current_row != row && current_col != col {
                    data.push(self.at(current_row, current_col));
                }
            }
        }

        data
    }
}

impl PartialEq for MatrixMethods<'_> {
    fn eq(&self, rhs: &MatrixMethods) -> bool {
        if self.size != rhs.size {
            return false;
        }

        for i in 0..(self.size * self.size) {
            if !is_same(self.data[i], rhs.data[i]) {
                return false;
            }
        }

        true
    }
}
