// This is a library to the code won't be used yet
#![allow(dead_code)]

// Code files
mod float_compare;
mod matrix2x2;
mod matrix3x3;
mod matrix4x4;
mod matrix_methods;
mod point;
mod tuple;
mod vector;

// Exports
pub use matrix2x2::Matrix2x2;
pub use matrix3x3::Matrix3x3;
pub use matrix4x4::Matrix4x4;
pub use point::Point;
pub use tuple::Tuple;
pub use vector::Vector;

// Tests
mod matrix_test;
mod tuple_test;
