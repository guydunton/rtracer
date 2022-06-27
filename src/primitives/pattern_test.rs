use crate::{image::Color, maths::Point};

use super::pattern::StripePattern;

const WHITE: Color = Color::white();
const BLACK: Color = Color::black();

#[test]
fn a_stripe_pattern_is_constant_in_y() {
    let pattern = StripePattern::new(WHITE, BLACK);

    assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.0)), WHITE);
}
