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

    for i in 0..2 {
        c.write_pixel(i, i, Color::red());
    }

    let save_buffer = c.get_save_buffer();
    let expected_buffer = vec![255, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 0, 0, 255];

    assert_eq!(save_buffer, expected_buffer);
}

fn create_canvas_for_sampling() -> Canvas {
    let mut canvas = Canvas::new(4, 4);
    canvas.write_pixel(0, 0, Color::red());
    canvas.write_pixel(1, 0, Color::red());
    canvas.write_pixel(2, 0, Color::green());
    canvas.write_pixel(3, 0, Color::green());
    canvas.write_pixel(0, 1, Color::red());
    canvas.write_pixel(1, 1, Color::red());
    canvas.write_pixel(2, 1, Color::green());
    canvas.write_pixel(3, 1, Color::green());
    canvas.write_pixel(0, 2, Color::blue());
    canvas.write_pixel(1, 2, Color::blue());
    canvas.write_pixel(2, 2, Color::red());
    canvas.write_pixel(3, 2, Color::red());
    canvas.write_pixel(0, 3, Color::blue());
    canvas.write_pixel(1, 3, Color::blue());
    canvas.write_pixel(2, 3, Color::red());
    canvas.write_pixel(3, 3, Color::red());

    canvas
}

#[test]
fn can_sample_canvas_pixel_for_2_x_2_sample() {
    /*
    r,r,g,g,
    r,r,g,g,
    b,b,r,r,
    b,b,r,r,
    */
    let canvas = create_canvas_for_sampling();

    // each sample is 2x2 canvas pixels averaged together

    // At top left color should be red
    let top_left = canvas.sample_at(0.25, 0.25, 2, 2);
    assert_eq!(top_left, Color::red());

    // at top right color should be green
    let top_right = canvas.sample_at(0.75, 0.25, 2, 2);
    assert_eq!(top_right, Color::green());

    // bottom left is blue
    let bottom_left = canvas.sample_at(0.25, 0.75, 2, 2);
    assert_eq!(bottom_left, Color::blue());

    // bottom right is red
    let bottom_right = canvas.sample_at(0.75, 0.75, 2, 2);
    assert_eq!(bottom_right, Color::red());
}

#[test]
fn can_downsample_canvas() {
    let canvas = create_canvas_for_sampling();

    let half_canvas = canvas.downsample(2, 2);
    assert_eq!(half_canvas.pixel_at(0, 0), Color::red());
    assert_eq!(half_canvas.pixel_at(1, 0), Color::green());
    assert_eq!(half_canvas.pixel_at(0, 1), Color::blue());
    assert_eq!(half_canvas.pixel_at(1, 1), Color::red());
}
