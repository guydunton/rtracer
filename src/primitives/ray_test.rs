#[cfg(test)]
mod ray_test {
    use super::super::{Intersection, Ray, Shape};
    use crate::{Matrix4x4, Point, Vector};
    use rust_catch::tests;

    tests! {
        test("Creating and querying a ray") {
            let origin = Point::new(1.0, 2.0, 3.0);
            let direction = Vector::new(4.0, 5.0, 6.0);

            let ray = Ray::new(origin, direction);

            assert_eq!(ray.origin(), origin);
            assert_eq!(ray.direction(), direction);
        }

        test("Computing a point from a distance") {
            let ray = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

            assert_eq!(ray.position(0.0), Point::new(2.0, 3.0, 4.0));
            assert_eq!(ray.position(1.0), Point::new(3.0, 3.0, 4.0));
            assert_eq!(ray.position(-1.0), Point::new(1.0, 3.0, 4.0));
            assert_eq!(ray.position(2.5), Point::new(4.5, 3.0, 4.0));
        }

        test("A ray intersects a sphere at 2 locations") {
            let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
            let s = Shape::sphere();

            let intersections = r.intersects(s);
            assert_eq!(intersections[0].t(), 4.0);
            assert_eq!(intersections[1].t(), 6.0);
        }

        test("A ray intersects a sphere at a tangent") {
            let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
            let s = Shape::sphere();
            let i = r.intersects(s);

            assert_eq!(i[0].t(), 5.0);
            assert_eq!(i[1].t(), 5.0);
        }

        test("A ray misses a sphere") {
            let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
            let s = Shape::sphere();

            let i = r.intersects(s);

            assert_eq!(i.len(), 0);
        }

        test("A ray originates inside a sphere") {
            let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
            let s = Shape::sphere();

            let i = r.intersects(s);
            assert_eq!(i[0].t(), -1.0);
            assert_eq!(i[1].t(), 1.0);
        }

        test("A sphere is behind a ray") {
            let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
            let s = Shape::sphere();

            let i = r.intersects(s);
            assert_eq!(i[0].t(), -6.0);
            assert_eq!(i[1].t(), -4.0);
            assert_eq!(i[0].object(), s);
            assert_eq!(i[1].object(), s);
        }

        test("An intersection encapsulates t and object") {
            let s = Shape::sphere();
            let i = Intersection::new(3.5, s);

            assert_eq!(i.t(), 3.5);
            assert_eq!(i.object(), s);
        }

        test("The hit when all intersections have positive t") {
            let s = Shape::sphere();
            let i1 = Intersection::new(1.0, s);
            let i2 = Intersection::new(2.0, s);

            let intersections = vec![i1, i2];
            let hit = Intersection::hit(intersections).unwrap();

            assert_eq!(hit, i1);
        }

        test("The hit when some intersections have negative t") {
            let s = Shape::sphere();
            let i1 = Intersection::new(-1.0, s);
            let i2 = Intersection::new(1.0, s);

            let intersections = vec![i1, i2];
            let hit = Intersection::hit(intersections).unwrap();

            assert_eq!(hit, i2);
        }

        test("The hit when all intersections have negative t") {
            let s = Shape::sphere();
            let i1 = Intersection::new(-2.0, s);
            let i2 = Intersection::new(-1.0, s);

            let intersections = vec![i1, i2];
            let hit = Intersection::hit(intersections);

            assert_eq!(hit, None);
        }

        test("The hit is always the lowest nonnegative intersection") {
            let s = Shape::sphere();
            let i1 = Intersection::new(5.0, s);
            let i2 = Intersection::new(7.0, s);
            let i3 = Intersection::new(-3.0, s);
            let i4 = Intersection::new(2.0, s);
            let hit = Intersection::hit(vec![i1, i2, i3, i4]).unwrap();

            assert_eq!(hit, i4);
        }

        test("Translate a ray") {
            let r1 = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
            let m = Matrix4x4::translation(3.0, 4.0, 5.0);
            let r2 = r1.transform(m);

            assert_eq!(r2.origin(), Point::new(4.0, 6.0, 8.0));
            assert_eq!(r2.direction(), Vector::new(0.0, 1.0, 0.0));
        }

        test("Scale a ray") {
            let r1 = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
            let m = Matrix4x4::scaling(2.0, 3.0, 4.0);
            let r2 = r1.transform(m);

            assert_eq!(r2.origin(), Point::new(2.0, 6.0, 12.0));
            assert_eq!(r2.direction(), Vector::new(0.0, 3.0, 0.0));
        }

        test("A spheres default transformation") {
            let s = Shape::sphere();
            assert_eq!(s.transformation(), Matrix4x4::identity());
        }

        test("Set a spheres transformation") {
            let t = Matrix4x4::translation(2.0, 3.0, 4.0);
            let s2 = Shape::sphere_from_transformation(t);

            assert_eq!(s2.transformation(), t);
        }

        test("Intersecting a scaled sphere with a ray") {
            let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
            let s = Shape::sphere_from_transformation(Matrix4x4::scaling(2.0, 2.0, 2.0));

            let intersections = r.intersects(s);

            assert_eq!(intersections.len(), 2);
            assert_eq!(intersections[0].t(), 3.0);
            assert_eq!(intersections[1].t(), 7.0);
        }

        test("Intersecting a translated sphere with a ray") {
            let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
            let s = Shape::sphere_from_transformation(Matrix4x4::translation(5.0, 0.0, 0.0));

            let intersections = r.intersects(s);

            assert_eq!(intersections.len(), 0);
        }
    }
}
