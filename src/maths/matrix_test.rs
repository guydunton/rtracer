#[cfg(test)]
mod matrix_test {
    use super::super::{Matrix2x2, Matrix3x3, Matrix4x4};
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

        test("Can construct a 3x3 matrix") {
            let m = Matrix3x3::new(
                -3.0, 5.0, 0.0,
                1.0, -2.0, -7.0,
                0.0, 1.0, 1.0
            );

            assert_eq!(m.at(0, 0), -3.0);
            assert_eq!(m.at(1, 1), -2.0);
            assert_eq!(m.at(2, 2), 1.0);
        }

        test("We can test the equality of matrices") {
            let a = Matrix4x4::new(
                1.0, 2.0, 3.0, 4.0,
                5.0, 6.0, 7.0, 8.0,
                9.0, 10.0, 11.0, 12.0,
                13.0, 14.0, 15.0, 16.0
            );
            let b = Matrix4x4::new(
                1.0, 2.0, 3.0, 4.0,
                5.0, 6.0, 7.0, 8.0,
                9.0, 10.0, 11.0, 12.0,
                13.0, 14.0, 15.0, 16.0,
            );

            let c = Matrix4x4::new(
                1.0, 2.0, 3.0, 4.0,
                5.0, 6.0, 7.0, 8.0,
                9.0, 8.0, 7.0, 6.0,
                5.0, 4.0, 3.0, 2.0,
            );

            assert_eq!(a, b);

            assert_ne!(a, c);
        }

        test("Matrices can be multiplied together") {
            let a = Matrix4x4::new(
                1.0, 2.0, 3.0, 4.0,
                5.0, 6.0, 7.0, 8.0,
                9.0, 8.0, 7.0, 6.0,
                5.0, 4.0, 3.0, 2.0,
            );

            let b = Matrix4x4::new(
                -2.0, 1.0, 2.0, 3.0,
                3.0, 2.0, 1.0, -1.0,
                4.0, 3.0, 6.0, 5.0,
                1.0, 2.0, 7.0, 8.0,
            );

            let expected = Matrix4x4::new(
                20.0, 22.0, 50.0, 48.0,
                44.0, 54.0, 114.0, 108.0,
                40.0, 58.0, 110.0, 102.0,
                16.0, 26.0, 46.0, 42.0,
            );

            assert_eq!(a * b, expected);
        }
    }
}
