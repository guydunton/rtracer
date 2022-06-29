use crate::{image::Color, maths::Point};

#[derive(Copy, Clone, Debug)]
pub struct StripePattern {
    col1: Color,
    col2: Color,
}

impl StripePattern {
    pub fn new(col1: Color, col2: Color) -> Self {
        Self { col1, col2 }
    }

    pub fn color_at(&self, point: Point) -> Color {
        if (point.x().floor() as i32 % 2) == 0 {
            return self.col1;
        } else {
            return self.col2;
        }
    }
}
