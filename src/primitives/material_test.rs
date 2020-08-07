#[cfg(test)]
mod material_test {
    use crate::{Color, Material, Point, PointLight, Vector};
    use rust_catch::tests;

    fn round(v: f64) -> f64 {
        const SIG_FIGS: f64 = 100000.0;
        (v * SIG_FIGS).round() / SIG_FIGS
    }

    tests! {
        test("Lighting with the eye between the light and the surface") {
            let m = Material::default();
            let position = Point::new(0.0, 0.0, 0.0);

            let eyev = Vector::new(0.0, 0.0, -1.0);
            let normalv = Vector::new(0.0, 0.0, -1.0);

            let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

            let result = m.lighting(light, position, eyev, normalv);
            assert_eq!(result, Color::new(1.9, 1.9, 1.9));
        }

        test("Lighting with the eye between light and surface eye offset 45") {
            let m = Material::default();
            let position = Point::new(0.0, 0.0, 0.0);
            let eyev = Vector::new(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
            let normalv = Vector::new(0.0, 0.0, -1.0);
            let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

            let result = m.lighting(light, position, eyev, normalv);
            assert_eq!(result, Color::new(1.0, 1.0, 1.0));
        }

        test("Lighting with eye opposite surface lighting offset 45") {
            let m = Material::default();
            let position = Point::new(0.0, 0.0, 0.0);

            let eyev = Vector::new(0.0, 0.0, -1.0);
            let normalv = Vector::new(0.0, 0.0, -1.0);
            let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

            let result = m.lighting(light, position, eyev, normalv);
            assert_eq!(round(result.r()), 0.7364);
            assert_eq!(round(result.g()), 0.7364);
            assert_eq!(round(result.b()), 0.7364);
        }

        test("Lighting with eye in the path of the reflection vector") {
            let m = Material::default();
            let position = Point::new(0.0, 0.0, 0.0);

            let eyev = Vector::new(0.0, -2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
            let normalv = Vector::new(0.0, 0.0, -1.0);
            let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

            let result = m.lighting(light, position, eyev, normalv);
            assert_eq!(round(result.r()), 1.6364);
            assert_eq!(round(result.g()), 1.6364);
            assert_eq!(round(result.b()), 1.6364);
        }

        test("Lighting with the light behind the surface") {
            let m = Material::default();
            let position = Point::new(0.0, 0.0, 0.0);

            let eyev = Vector::new(0.0, 0.0, -1.0);
            let normalv = Vector::new(0.0, 0.0, -1.0);
            let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

            let result = m.lighting(light, position, eyev, normalv);
            assert_eq!(result, Color::new(0.1, 0.1, 0.1));
        }
    }
}
