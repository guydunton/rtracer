use super::matrix_methods::MatrixMethods;

pub struct Matrix2x2 {
    data: [f64; 4],
}

impl Matrix2x2 {
    pub fn new(m00: f64, m01: f64, m10: f64, m11: f64) -> Matrix2x2 {
        Matrix2x2 {
            data: [m00, m01, m10, m11],
        }
    }

    pub fn at(&self, row: usize, col: usize) -> f64 {
        let props = MatrixMethods::new(&self.data, 2);
        props.at(row, col)
    }
}
