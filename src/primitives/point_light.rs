use crate::canvas::Color;
use crate::maths::Point;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PointLight {
    intensity: Color,
    position: Point,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> Self {
        Self {
            intensity,
            position,
        }
    }

    pub fn intensity(&self) -> Color {
        self.intensity
    }

    pub fn position(&self) -> Point {
        self.position
    }
}

#[test]
fn test_point_light_creation() {
    let intensity = Color::new(1.0, 1.0, 1.0);
    let position = Point::new(0.0, 0.0, 0.0);

    let light = PointLight::new(position, intensity);
    assert_eq!(light.position(), position);
    assert_eq!(light.intensity(), intensity);
}
