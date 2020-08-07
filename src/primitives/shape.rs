use super::Material;
use crate::maths::{Matrix4x4, Point, Vector};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    transform: Matrix4x4,
    material: Material,
}

impl Sphere {
    pub fn new(transform: Matrix4x4, material: Material) -> Self {
        Self {
            transform,
            material,
        }
    }

    pub fn sphere_from_transformation(m: Matrix4x4) -> Self {
        Self {
            transform: m,
            material: Material::default(),
        }
    }

    pub fn default() -> Self {
        Self {
            transform: Matrix4x4::identity(),
            material: Material::default(),
        }
    }

    pub fn transformation(&self) -> Matrix4x4 {
        self.transform
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn normal_at(&self, p: Point) -> Vector {
        let inverse = self.transformation().inverse().unwrap();

        let object_point = inverse * p;
        let object_normal = object_point - Point::new(0.0, 0.0, 0.0);
        let mut world_normal = inverse.transpose() * object_normal;
        world_normal.set_w(0.0);
        world_normal.normalize()
    }
}
