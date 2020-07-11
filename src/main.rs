mod canvas;
mod maths;

use canvas::{save_canvas, Canvas, Color};
use maths::{Matrix4x4, Tuple};

fn main() {
    let mut canvas = Canvas::new(128, 128);

    let point = Tuple::point(0.0, 0.0, 0.0);

    for i in 0..12 {
        let m = Matrix4x4::identity()
            .translate(0.0, 50.0, 0.0)
            .rotate_z(std::f64::consts::FRAC_PI_6 * i as f64)
            .translate(128.0 / 2.0, 128.0 / 2.0, 0.0);

        let result: Tuple = m * point;

        canvas.write_pixel(result.x() as i32, result.y() as i32, Color::white());
    }

    save_canvas(canvas, "out.png".to_owned()).unwrap();
}
