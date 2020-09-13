use super::Material;
use crate::maths::{Matrix4x4, Point, Vector};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShapeType {
    Sphere,
    Plane,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Shape {
    shape_type: ShapeType,
    transform: Matrix4x4,
    material: Material,
    transformation_inverse: Matrix4x4,
}

impl Shape {
    pub fn sphere_from_transformation(transform: Matrix4x4) -> Self {
        Self {
            shape_type: ShapeType::Sphere,
            transform: transform,
            material: Material::default(),
            transformation_inverse: transform.inverse().unwrap(),
        }
    }

    pub fn sphere(transform: Matrix4x4, material: Material) -> Self {
        Self {
            shape_type: ShapeType::Sphere,
            transform,
            material,
            transformation_inverse: transform.inverse().unwrap(),
        }
    }

    pub fn plane_default() -> Shape {
        Shape {
            shape_type: ShapeType::Plane,
            transform: Matrix4x4::identity(),
            transformation_inverse: Matrix4x4::identity().inverse().unwrap(),
            material: Material::default(),
        }
    }

    pub fn default() -> Self {
        Self {
            shape_type: ShapeType::Sphere,
            transform: Matrix4x4::identity(),
            material: Material::default(),
            transformation_inverse: Matrix4x4::identity().inverse().unwrap(),
        }
    }

    pub fn shape_type(&self) -> ShapeType {
        self.shape_type
    }

    pub fn transformation(&self) -> Matrix4x4 {
        self.transform
    }

    pub fn transformation_inverse(&self) -> Matrix4x4 {
        self.transformation_inverse
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn normal_at(&self, p: Point) -> Vector {
        let inverse = self.transformation_inverse();
        let object_point = inverse * p;

        // Find the local normal for each shape type
        let object_normal = match self.shape_type {
            ShapeType::Sphere => object_point - Point::new(0.0, 0.0, 0.0),
            ShapeType::Plane => Vector::new(0.0, 1.0, 0.0),
        };

        let mut world_normal = inverse.transpose() * object_normal;
        world_normal.set_w(0.0);
        world_normal.normalize()
    }
}
