use crate::{image::Color, maths::Point};

pub struct StripePattern {}

impl StripePattern {
    pub fn new(_col1: Color, _col2: Color) -> Self {
        Self {}
    }

    pub fn color_at(&self, _point: Point) -> Color {
        Color::white()
    }
}
