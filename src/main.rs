mod canvas;
mod maths;

use canvas::{Canvas, Color, save_canvas};

fn main() {
    let mut canvas = Canvas::new(128, 128);
    let red = Color::new(1.0, 0.0, 0.0);

    for i in 0..128 {
        canvas.write_pixel(i, i, red);
    }

    save_canvas(canvas, "out.png".to_owned()).unwrap();
}
