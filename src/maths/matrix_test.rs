#[cfg(test)]
mod matrix_test {
    use super::super::{Matrix2x2, Matrix4x4};
    use rust_catch::tests;

    tests! {
        test("Can construct a matrix4x4") {
            let m = Matrix4x4::new(
                1.0, 2.0, 3.0, 4.0,
                5.5, 6.5, 7.5, 8.5,
                9.0, 10.0, 11.0, 12.0,
                13.5, 14.5, 15.5, 16.5
            );

            assert_eq!(m.at(0, 0), 1.0);
            assert_eq!(m.at(0, 3), 4.0);
            assert_eq!(m.at(1, 0), 5.5);
        }

        test("Can construct a 2x2 matrix") {
            let m = Matrix2x2::new(
                -3.0, 5.0,
                1.0, -2.0
            );

            assert_eq!(m.at(0, 0), -3.0);
            assert_eq!(m.at(0, 1), 5.0);
            assert_eq!(m.at(1, 0), 1.0);
            assert_eq!(m.at(1, 1), -2.0);
        }
    }
}
