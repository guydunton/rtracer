// This is a library to the code won't be used yet
#![allow(dead_code)]

// Code files
mod canvas;
mod canvas_writer;
mod color;

// Exports
pub use canvas::Canvas;
pub use canvas_writer::save_canvas;
pub use color::Color;

// Tests
#[cfg(test)]
mod canvas_test;
#[cfg(test)]
mod color_test;
