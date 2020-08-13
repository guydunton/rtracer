use super::Material;
use crate::maths::{Matrix4x4, Point, Vector};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    transform: Matrix4x4,
    material: Material,
    transformation_inverse: Matrix4x4,
}

impl Sphere {
    pub fn new(transform: Matrix4x4, material: Material) -> Self {
        Self {
            transform,
            material,
            transformation_inverse: transform.inverse().unwrap(),
        }
    }

    pub fn sphere_from_transformation(m: Matrix4x4) -> Self {
        Self {
            transform: m,
            material: Material::default(),
            transformation_inverse: m.inverse().unwrap(),
        }
    }

    pub fn default() -> Self {
        Self {
            transform: Matrix4x4::identity(),
            material: Material::default(),
            transformation_inverse: Matrix4x4::identity().inverse().unwrap(),
        }
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
        let object_normal = object_point - Point::new(0.0, 0.0, 0.0);
        let mut world_normal = inverse.transpose() * object_normal;
        world_normal.set_w(0.0);
        world_normal.normalize()
    }
}
