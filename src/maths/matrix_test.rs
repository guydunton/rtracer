#[cfg(test)]
mod matrix_test {
    use super::super::{Matrix2x2, Matrix3x3, Matrix4x4, Tuple};
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

        test("Matrices can be multiplied by a tuple") {
            let m = Matrix4x4::new(
                1.0, 2.0, 3.0, 4.0,
                2.0, 4.0, 4.0, 2.0,
                8.0, 6.0, 4.0, 1.0,
                0.0, 0.0, 0.0, 1.0
            );

            let t = Tuple::new(1.0, 2.0, 3.0, 1.0);

            assert_eq!(m * t, Tuple::new(18.0, 24.0, 33.0, 1.0));
        }

        test("Multiplying a by identity returns a") {
            let a = Matrix4x4::new(
                0.0, 1.0, 2.0, 4.0,
                1.0, 2.0, 4.0, 8.0,
                2.0, 4.0, 8.0, 16.0,
                4.0, 8.0, 16.0, 32.0
            );

            let id = Matrix4x4::identity();
            assert_eq!(a * id, a);
        }

        test("Matrices can be transposed") {
            let a = Matrix4x4::new(
                0.0, 9.0, 3.0, 0.0,
                9.0, 8.0, 0.0, 8.0,
                1.0, 8.0, 5.0, 3.0,
                0.0, 0.0, 5.0, 8.0,
            );

            let expected = Matrix4x4::new(
                0.0, 9.0, 1.0, 0.0,
                9.0, 8.0, 8.0, 0.0,
                3.0, 0.0, 5.0, 5.0,
                0.0, 8.0, 3.0, 8.0,
            );

            assert_eq!(a.transpose(), expected);
        }

        test("We can calculate the determinant of a 2x2 matrix") {
            let m = Matrix2x2::new(
                1.0, 5.0,
                -3.0, 2.0
            );

            assert_eq!(m.determinant(), 17.0);
        }

        test("Create a submatrix from a 3x3 matrix") {
            let m = Matrix3x3::new(
                1.0, 5.0, 0.0,
                -3.0, 2.0, 7.0,
                0.0, 6.0, -3.0
            );

            let result = Matrix2x2::new(
                -3.0, 2.0,
                0.0, 6.0
            );

            assert_eq!(m.submatrix(0, 2), result);
        }
    }
}
