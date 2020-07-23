use crate::maths::Matrix4x4;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Sphere { transform: Matrix4x4 },
}

impl Shape {
    pub fn sphere() -> Shape {
        Shape::Sphere {
            transform: Matrix4x4::identity(),
        }
    }

    pub fn sphere_from_transformation(m: Matrix4x4) -> Shape {
        Shape::Sphere { transform: m }
    }

    pub fn transformation(&self) -> Matrix4x4 {
        match *self {
            Shape::Sphere { transform } => transform,
        }
    }
}
