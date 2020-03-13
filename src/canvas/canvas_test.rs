#[cfg(test)]
mod canvas_test {
    use super::super::Canvas;
    use super::super::Color;
    use rust_catch::tests;

    tests! {
        test("Can create a canvas") {
            let c = Canvas::new(10, 20);
            assert_eq!(c.width(), 10);
            assert_eq!(c.height(), 20);
        }

        test("Canvas pixels be set to a color") {
            let mut c = Canvas::new(10, 20);
            let red = Color::new(1.0, 0.0, 0.0);
            c.write_pixel(2, 3, red);

            assert_eq!(c.pixel_at(2, 3), red);
        }

        test("Canvas can be written to png") {
            let mut c = Canvas::new(2, 2);
            let red = Color::new(1.0, 0.0, 0.0);

            for i in 0..2 {
                c.write_pixel(i, i, red);
            }

            let save_buffer = c.get_save_buffer();
            let expected_buffer = vec![
                255, 0, 0, 255, 
                0, 0, 0, 255, 
                0, 0, 0, 255,
                255, 0, 0, 255, 
            ];

            assert_eq!(save_buffer, expected_buffer);
        }
    }
}