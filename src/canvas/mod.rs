mod color;
mod canvas;
mod canvas_writer;

// Exports
pub use color::Color;
pub use canvas::Canvas;
pub use canvas_writer::save_canvas;

// Tests
mod color_test;
mod canvas_test;
