#[cfg(test)]
mod matrix_test {
    use super::super::{Matrix2x2, Matrix3x3, Matrix4x4, Tuple};
    use crate::maths::{Point, Vector};
    use rust_catch::tests;
    use std::f64::consts::PI;

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

        test("Calculate the inverse of a third matrix") {
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

        test("Multiplying by a translation matrix") {
            let transform = Matrix4x4::translation(5.0, -3.0, 2.0);
            let p = Tuple::point(-3.0, 4.0, 5.0);

            assert_eq!(transform * p, Tuple::point(2.0, 1.0, 7.0));
        }

        test("Multiplying by the inverse of translation matrix") {
            let transform = Matrix4x4::translation(5.0, -3.0, 2.0);
            let p = Tuple::point(-3.0, 4.0, 5.0);
            let inv = transform.inverse().unwrap();

            assert_eq!(inv * p, Tuple::point(-8.0, 7.0, 3.0));
        }

        test("Translation does not affect vectors") {
            let transform = Matrix4x4::translation(5.0, -3.0, 2.0);
            let v = Tuple::vector(-3.0, 4.0, 5.0);

            assert_eq!(transform * v, v);
        }

        test("apply a scaling matrix to a point") {
            let transform = Matrix4x4::scaling(2.0, 3.0, 4.0);
            let p = Tuple::point(-4.0, 6.0, 8.0);

            assert_eq!(transform * p, Tuple::point(-8.0, 18.0, 32.0));
        }

        test("apply a scaling matrix to a vector") {
            let transform = Matrix4x4::scaling(2.0, 3.0, 4.0);
            let p = Tuple::vector(-4.0, 6.0, 8.0);

            assert_eq!(transform * p, Tuple::vector(-8.0, 18.0, 32.0));
        }

        test("Multiply by the inverse of a scaling matrix") {
            let transform = Matrix4x4::scaling(2.0, 3.0, 4.0);
            let inv = transform.inverse().unwrap();
            let v = Tuple::vector(-4.0, 6.0, 8.0);

            assert_eq!(inv * v, Tuple::vector(-2.0, 2.0, 2.0));
        }

        test("reflection is scaling by a negative value") {
            let transform = Matrix4x4::scaling(-1.0, 1.0, 1.0);
            let p = Tuple::point(2.0, 3.0, 4.0);

            assert_eq!(transform * p, Tuple::point(-2.0, 3.0, 4.0));
        }

        test("Rotating a point around the x axis") {
            let p = Tuple::point(0.0, 1.0, 0.0);
            let half_quarter = Matrix4x4::rotation_x(PI / 4.0);
            let full_quarter = Matrix4x4::rotation_x(PI / 2.0);

            assert_eq!(half_quarter * p, Tuple::point(0.0, 2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0));
            assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
        }

        test("Inverse of x rotation works in opposite direction") {
            let p = Tuple::point(0.0, 1.0, 0.0);
            let half_quarter = Matrix4x4::rotation_x(PI / 4.0);
            let inverse = half_quarter.inverse().unwrap();

            assert_eq!(inverse * p, Tuple::point(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0));
        }

        test("Rotate around the y axis") {
            let p = Tuple::point(0.0, 0.0, 1.0);
            let half_quarter = Matrix4x4::rotation_y(PI / 4.0);
            let full_quarter = Matrix4x4::rotation_y(PI / 2.0);

            assert_eq!(half_quarter * p, Tuple::point(2f64.sqrt() / 2.0, 0.0, 2f64.sqrt() / 2.0));
            assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
        }

        test("Rotate around the z axis") {
            let p = Tuple::point(0.0, 1.0, 0.0);
            let half_quarter = Matrix4x4::rotation_z(PI / 4.0);
            let full_quarter = Matrix4x4::rotation_z(PI / 2.0);

            assert_eq!(half_quarter * p, Tuple::point(-2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0, 0.0));
            assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
        }

        test("A shearing transformation moves x in proportion to y") {
            let transform = Matrix4x4::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            let p = Tuple::point(2.0, 3.0, 4.0);

            assert_eq!(transform * p, Tuple::point(5.0, 3.0, 4.0));
        }

        test("Individual transformations are applied in sequence") {
            let p = Tuple::point(1.0, 0.0, 1.0);
            let A = Matrix4x4::rotation_x(std::f64::consts::PI / 2.0);
            let B = Matrix4x4::scaling(5.0, 5.0, 5.0);
            let C = Matrix4x4::translation(10.0, 5.0, 7.0);

            let p2 = A * p;
            assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));
            let p3 = B * p2;
            assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));
            let p4 = C * p3;
            assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
        }

        test("Chained transformations must be applied in reverse order") {
            let p = Tuple::point(1.0, 0.0, 1.0);
            let A = Matrix4x4::rotation_x(std::f64::consts::PI / 2.0);
            let B = Matrix4x4::scaling(5.0, 5.0, 5.0);
            let C = Matrix4x4::translation(10.0, 5.0, 7.0);

            let T = C * B * A;
            assert_eq!(T * p, Tuple::point(15.0, 0.0, 7.0));
        }

        test("Chain transformation using fluent api") {
            let p = Tuple::point(1.0, 0.0, 1.0);
            let T = Matrix4x4::identity()
                .translate(10.0, 5.0, 7.0)
                .scale(5.0, 5.0, 5.0)
                .rotate_x(std::f64::consts::PI / 2.0);
            assert_eq!(T * p, Tuple::point(15.0, 0.0, 7.0));
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

    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::up();

        let view = Matrix4x4::view(from, to, up);
        assert_eq!(view, Matrix4x4::identity());
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::up();

        let view = Matrix4x4::view(from, to, up);
        assert_eq!(view, Matrix4x4::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::up();

        let view = Matrix4x4::view(from, to, up);
        assert_eq!(view, Matrix4x4::translation(0.0, 0.0, -8.0));
    }
}
