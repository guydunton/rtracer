use crate::{image::Color, maths::Point};

use super::pattern::StripePattern;

const WHITE: Color = Color::white();
const BLACK: Color = Color::black();

#[test]
fn a_stripe_pattern_is_constant_in_y() {
    let pattern = StripePattern::new(WHITE, BLACK);
    assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.color_at(Point::new(0.0, 1.0, 0.0)), WHITE);
    assert_eq!(pattern.color_at(Point::new(0.0, 2.0, 0.0)), WHITE);
}

#[test]
fn a_stripe_pattern_is_constant_in_z() {
    let pattern = StripePattern::new(WHITE, BLACK);
    assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 1.0)), WHITE);
    assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 2.0)), WHITE);
}

#[test]
fn a_stripe_pattern_alternates_in_x() {
    let pattern = StripePattern::new(WHITE, BLACK);
    assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.color_at(Point::new(0.9, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.color_at(Point::new(1.0, 0.0, 0.0)), BLACK);
    assert_eq!(pattern.color_at(Point::new(-0.1, 0.0, 0.0)), BLACK);
    assert_eq!(pattern.color_at(Point::new(-1.0, 0.0, 0.0)), BLACK);
    assert_eq!(pattern.color_at(Point::new(-1.1, 0.0, 0.0)), WHITE);
}
