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

        test("Get the minor of a matrix3x3") {
            let a = Matrix3x3::new(
                3.0, 5.0, 0.0,
                2.0, -1.0, -7.0,
                6.0, -1.0, 5.0
            );

            let b = a.submatrix(1, 0);
            assert_eq!(b.determinant(), 25.0);
            assert_eq!(a.minor(1, 0), 25.0);
        }

        test("Calculate the cofactor of a 3x3 matrix") {
            let a = Matrix3x3::new(
                3.0, 5.0, 0.0,
                2.0, -1.0, -7.0,
                6.0, -1.0, 5.0
            );

            assert_eq!(a.minor(0, 0), -12.0);
            assert_eq!(a.cofactor(0, 0), -12.0);
            assert_eq!(a.minor(1, 0), 25.0);
            assert_eq!(a.cofactor(1, 0), -25.0);
        }

        test("Calculate the determinant of a 3x3 matrix") {
            let a = Matrix3x3::new(
                1.0, 2.0, 6.0,
                -5.0, 8.0, -4.0,
                2.0, 6.0, 4.0
            );

            assert_eq!(a.cofactor(0, 0), 56.0);
            assert_eq!(a.cofactor(0, 1), 12.0);
            assert_eq!(a.cofactor(0, 2), -46.0);
            assert_eq!(a.determinant(), -196.0);
        }

        test("Calculate the determinant of a 4x4 matrix") {
            let a = Matrix4x4::new(
                -2.0, -8.0, 3.0, 5.0,
                -3.0, 1.0, 7.0, 3.0,
                1.0, 2.0, -9.0, 6.0,
                -6.0, 7.0, 7.0, -9.0
            );

            assert_eq!(a.cofactor(0, 0), 690.0);
            assert_eq!(a.cofactor(0, 1), 447.0);
            assert_eq!(a.cofactor(0, 2), 210.0);
            assert_eq!(a.cofactor(0, 3), 51.0);
            assert_eq!(a.determinant(), -4071.0);
        }

        test("Testing matrix invertability") {
            let invertable = Matrix4x4::new(
                6.0, 4.0, 4.0, 4.0,
                5.0, 5.0, 7.0, 6.0,
                4.0, -9.0, 3.0, -7.0,
                9.0, 1.0, 7.0, -6.0
            );

            assert_eq!(invertable.determinant(), -2120.0);
            assert_eq!(invertable.is_invertable(), true);

            let not_invertable = Matrix4x4::new(
                -4.0, 2.0, -2.0, -3.0,
                9.0, 6.0, 2.0, 6.0,
                0.0, -5.0, 1.0, -5.0,
                0.0, 0.0, 0.0, 0.0
            );

            assert_eq!(not_invertable.determinant(), 0.0);
            assert_eq!(not_invertable.is_invertable(), false);
        }

        test("Calculate the inverse of a matrix") {
            let a = Matrix4x4::new(
                -5.0, 2.0, 6.0, -8.0,
                1.0, -5.0, 1.0, 8.0,
                7.0, 7.0, -6.0, -7.0,
                1.0, -3.0, 7.0, 4.0
            );

            let mut b = a.inverse().unwrap();

            assert_eq!(a.determinant(), 532.0);
            assert_eq!(a.cofactor(2, 3), -160.0);
            assert_eq!(b.at(3, 2), -160.0/532.0);
            assert_eq!(a.cofactor(3, 2), 105.0);
            assert_eq!(b.at(2, 3), 105.0/532.0);

            let expected = Matrix4x4::new(
                0.21805, 0.45113, 0.24060, -0.04511,
                -0.80827, -1.45677, -0.44361, 0.52068,
                -0.07895, -0.22368, -0.05263, 0.19737,
                -0.52256, -0.81391, -0.30075, 0.30639
            );

            round_matrix(&mut b);
            assert_eq!(b, expected);
        }

        test("Calculate the inverse of a second matrix") {
            let a = Matrix4x4::new(
                8.0, -5.0, 9.0, 2.0,
                7.0, 5.0, 6.0, 1.0,
                -6.0, 0.0, 9.0, 6.0,
                -3.0, 0.0, -9.0, -4.0
            );

            let expected = Matrix4x4::new(
                -0.15385, -0.15385, -0.28205, -0.53846,
                -0.07692,  0.12308,  0.02564,  0.03077,
                 0.35897,  0.35897,  0.43590,  0.92308,
                -0.69231, -0.69231, -0.76923, -1.92308
            );

            let mut b = a.inverse().unwrap();
            round_matrix(&mut b);

            assert_eq!(b, expected);
        }

        test("Calculat the inverse of a third matrix") {
            let a = Matrix4x4::new(
                9.0,  3.0,  0.0,  9.0,
                -5.0, -2.0, -6.0, -3.0,
                -4.0,  9.0,  6.0,  4.0,
                -7.0,  6.0,  6.0,  2.0,
            );

            let expected = Matrix4x4::new(
                -0.04074, -0.07778,  0.14444, -0.22222 ,
                -0.07778,  0.03333,  0.36667, -0.33333 ,
                -0.02901, -0.14630, -0.10926,  0.12963 ,
                0.17778,  0.06667, -0.26667,  0.33333
            );

            let mut b = a.inverse().unwrap();
            round_matrix(&mut b);

            assert_eq!(b, expected);
        }

        test("Multiplying a product by its invers") {
            let a = Matrix4x4::new(
                3.0, -9.0, 7.0, 3.0,
                3.0, -8.0, 2.0, -9.0,
                -4.0, 4.0, 4.0, 1.0,
                -6.0, 5.0, -1.0, 1.0
            );

            let b = Matrix4x4::new(
                8.0, 2.0, 2.0, 2.0,
                3.0, -1.0, 7.0, 0.0,
                7.0, 0.0, 5.0, 4.0,
                6.0, -2.0, 0.0, 5.0
            );

            let c = a * b;
            assert_eq!(c * b.inverse().unwrap(), a);
        }
    }

    fn round_matrix(matrix: &mut Matrix4x4) {
        let round = |val: f64| (val * 100000.0).round() / 100000.0;

        for row in 0..4 {
            for col in 0..4 {
                matrix.set_at(row, col, round(matrix.at(row, col)));
            }
        }
    }
}
