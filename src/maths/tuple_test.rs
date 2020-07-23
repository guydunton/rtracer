#[cfg(test)]
mod tuple_test {
    use super::super::point::Point;
    use super::super::vector::Vector;
    use super::super::Tuple;
    use rust_catch::tests;

    tests! {
        test("can create a tuple and get its values") {
            let t = Tuple::new(1.0, 2.0, 3.0, 4.0);
            assert_eq!(t.x(), 1.0);
            assert_eq!(t.y(), 2.0);
            assert_eq!(t.z(), 3.0);
            assert_eq!(t.w(), 4.0);
        }

        test("tuple with w 1 is a vector") {
            let t = Tuple::new(1.0, 2.0, 3.0, 0.0);
            assert_eq!(t.is_vector(), true);
            assert_eq!(t.is_point(), false);
        }

        test("tuple with w 0 is a point") {
            let t = Tuple::new(1.0, 2.0, 3.0, 1.0);
            assert_eq!(t.is_point(), true);
            assert_eq!(t.is_vector(), false);
        }

        test("point creates a tuple with w 1") {
            let p = Point::new(1.0, 2.0, 3.0);
            assert_eq!(p.w(), 1.0);
        }

        test("vector create a tuple with w 0") {
            let v = Vector::new(1.0, 2.0, 3.0);
            assert_eq!(v.w(), 0.0);
        }

        test("tuples compare correctly") {
            let a = Tuple::new(1.0, 2.0, 3.0, 0.0);
            let b = Tuple::new(1.0, 2.0, 3.0, 0.0);

            assert_eq!(a, b);
        }

        test("adding tuples works") {
            let a = Tuple::new(3.0, -2.0, 5.0, 1.0);
            let b = Tuple::new(-2.0, 3.0, 1.0, 0.0);

            assert_eq!(a + b, Tuple::new(1.0, 1.0, 6.0, 1.0));
        }

        test("subtracting 2 points") {
            let p1 = Point::new(3.0, 2.0, 1.0);
            let p2 = Point::new(5.0, 6.0, 7.0);

            assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
        }

        test("subtracting 2 actual points") {
            let p1 = Point::new(3.0, 2.0, 1.0);
            let p2 = Point::new(5.0, 6.0, 7.0);

            assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
        }

        test("subtracting a vector from a point") {
            let p = Tuple::point(3.0, 2.0, 1.0);
            let v = Tuple::vector(5.0, 6.0, 7.0);

            assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
        }

        test("subtracting a real vector from a real point") {
            let p = Point::new(3.0, 2.0, 1.0);
            let v = Vector::new(5.0, 6.0, 7.0);

            assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0));
        }

        test("subtracting 2 vectors") {
            let v1 = Tuple::vector(3.0, 2.0, 1.0);
            let v2 = Tuple::vector(5.0, 6.0, 7.0);

            assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
        }

        test("subtracting 2 actual vectors") {
            let v1 = Vector::new(3.0, 2.0, 1.0);
            let v2 = Vector::new(5.0, 6.0, 7.0);

            assert_eq!(v1 - v2, Vector::new(-2.0, -4.0, -6.0));
        }

        test("tuples can be negated") {
            let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
            assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
        }

        test("Tuple can be multiplied by a scaler") {
            let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
            assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
        }

        test("Tuple can be divided by a scaler") {
            let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
            assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
        }

        test("Compute the length of a vector") {
            let v = Vector::new(1.0, 0.0, 0.0);
            assert_eq!(v.len(), 1.0);
        }

        test("Compute the length of a different vector") {
            let v = Vector::new(1.0, 2.0, 3.0);
            assert_eq!(v.len(), f64::sqrt(14.0));
        }

        test("normalize vector 4_0_0 gives 1_0_0") {
            let v = Vector::new(4.0, 0.0, 0.0);
            assert_eq!(v.normalize(), Vector::new(1.0, 0.0, 0.0));
        }

        test("Compute the dot product of 2 vectors") {
            let v1 = Vector::new(1.0, 2.0, 3.0);
            let v2 = Vector::new(2.0, 3.0, 4.0);
            assert_eq!(Vector::dot(v1, v2), 20.0);
        }

        test("Compute the cross product of 2 vectors") {
            let v1 = Vector::new(1.0, 2.0, 3.0);
            let v2 = Vector::new(2.0, 3.0, 4.0);
            assert_eq!(Vector::cross(v1, v2), Vector::new(-1.0, 2.0, -1.0));
            assert_eq!(Vector::cross(v2, v1), Vector::new(1.0, -2.0, 1.0));
        }
    }
}
