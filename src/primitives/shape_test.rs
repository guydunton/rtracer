#[cfg(test)]
mod shape_test {
    use super::super::Sphere;
    use crate::{Material, Matrix4x4, Point, Vector};
    use rust_catch::tests;
    use std::f64::consts::PI;

    fn round(v: f64) -> f64 {
        const SIG_FIGS: f64 = 100000.0;
        (v * SIG_FIGS).round() / SIG_FIGS
    }

    tests! {
        test("The normal on different axis") {
            let s = Sphere::default();
            let n1 = s.normal_at(Point::new(1.0, 0.0, 0.0));
            assert_eq!(n1, Vector::new(1.0, 0.0, 0.0));
            let n2 = s.normal_at(Point::new(0.0, 1.0, 0.0));
            assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
            let n3 = s.normal_at(Point::new(0.0, 0.0, 1.0));
            assert_eq!(n3, Vector::new(0.0, 0.0, 1.0));

            let p = 3f64.sqrt() / 3.0;
            let n4 = s.normal_at(Point::new(p, p, p));
            assert_eq!(n4, Vector::new(p, p, p));
        }

        test("The normal is a normalized vector") {
            let s = Sphere::default();
            let p = 3f64.sqrt() / 3.0;
            let n = s.normal_at(Point::new(p, p, p));
            assert_eq!(n, n.normalize());
        }

        test("Computing the normal on a translated sphere") {
            let s = Sphere::sphere_from_transformation(Matrix4x4::translation(0.0, 1.0, 0.0));
            let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));

            assert_eq!(round(n.x()), 0.0);
            assert_eq!(round(n.y()), 0.70711);
            assert_eq!(round(n.z()), -0.70711);
            assert_eq!(round(n.w()), 0.0);
        }

        test("Computing the normal on a transformed sphere") {
            //let m = Matrix4x4::scaling(1.0, 0.5, 1.0).rotate_z(PI / 5.0);
            let m = Matrix4x4::scaling(1.0, 0.5, 1.0).rotate_z(PI / 5.0);
            let s = Sphere::sphere_from_transformation(m);

            let p = 2f64.sqrt() / 2.0;
            let n = s.normal_at(Point::new(0.0, p, -p));
            assert_eq!(round(n.x()), 0.0);
            assert_eq!(round(n.y()), 0.97014);
            assert_eq!(round(n.z()), -0.24254);
            assert_eq!(round(n.w()), 0.0);
        }

        test("A sphere has a default material") {
            let s = Sphere::default();
            assert_eq!(s.material(), Material::default());
        }

        test("A sphere may be assigned a material") {
            let mut m = Material::default();
            m.ambient = 1.0;
            let s = Sphere::new(Matrix4x4::identity(), m);
            assert_eq!(s.material(), m);
        }
    }
}
