#[cfg(test)]
mod color_test {
    use super::super::Color;
    use rust_catch::tests;

    tests! {
        test("Retrieving the components of a color works") {
            let c = Color::new(-0.5, 0.4, 1.7);
            assert_eq!(c.r(), -0.5);
            assert_eq!(c.g(), 0.4);
            assert_eq!(c.b(), 1.7);

            assert_eq!(c, Color::new(-0.5, 0.4, 1.7));
        }

        test("Colors can be added") {
            let c1 = Color::new(0.9, 0.6, 0.75);
            let c2 = Color::new(0.7, 0.1, 0.25);

            assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
        }

        test("Colors can be subtracted") {
            let c1 = Color::new(0.9, 0.6, 0.75);
            let c2 = Color::new(0.7, 0.1, 0.25);

            assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
        }

        test("Colors can be multiplied by scalar") {
            let c = Color::new(0.2, 0.3, 0.4);
            assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
        }

        test("Colors can be multiplied together") {
            let c1 = Color::new(1.0, 0.2, 0.4);
            let c2 = Color::new(0.9, 1.0, 0.1);

            assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
        }
    }
}
