use super::super::maths::Tuple;
use std::cmp::PartialEq;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    data: Tuple,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color {
            data: Tuple::new(red, green, blue, 0.0),
        }
    }

    pub fn r(&self) -> f64 {
        self.data.x()
    }

    pub fn g(&self) -> f64 {
        self.data.y()
    }

    pub fn b(&self) -> f64 {
        self.data.z()
    }
}

impl PartialEq for Color {
    fn eq(&self, rhs: &Color) -> bool {
        self.data == rhs.data
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Color) -> Color {
        Color {
            data: self.data + rhs.data,
        }
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Color) -> Color {
        Color {
            data: self.data - rhs.data,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Color {
        Color {
            data: self.data * rhs,
        }
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Color) -> Color {
        Color::new(self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b())
    }
}
