use super::matrix_methods::MatrixMethods;

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
        let methods = MatrixMethods::new(&self.data, 4);
        methods.at(row, col)
    }
}
