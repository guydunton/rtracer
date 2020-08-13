use super::{Canvas, Color};

#[test]
fn can_create_a_canvas() {
    let c = Canvas::new(10, 20);
    assert_eq!(c.width(), 10);
    assert_eq!(c.height(), 20);
}

#[test]
fn canvas_pixels_be_set_to_a_color() {
    let mut c = Canvas::new(10, 20);
    let red = Color::new(1.0, 0.0, 0.0);
    c.write_pixel(2, 3, red);

    assert_eq!(c.pixel_at(2, 3), red);
}

#[test]
fn canvas_can_be_written_to_png() {
    let mut c = Canvas::new(2, 2);
    let red = Color::new(1.0, 0.0, 0.0);

    for i in 0..2 {
        c.write_pixel(i, i, red);
    }

    let save_buffer = c.get_save_buffer();
    let expected_buffer = vec![255, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 0, 0, 255];

    assert_eq!(save_buffer, expected_buffer);
}
