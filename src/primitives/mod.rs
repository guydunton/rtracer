// This is a library to the code won't be used yet
#![allow(dead_code)]

// Code
mod camera;
mod intersection;
mod intersection_stats;
mod material;
mod pattern;
mod point_light;
mod ray;
mod shape;
mod world;

// Exports
pub use camera::Camera;
pub use intersection::Intersection;
pub use intersection_stats::IntersectionStats;
pub use material::Material;
pub use pattern::StripePattern;
pub use point_light::PointLight;
pub use ray::Ray;
pub use shape::Shape;
pub use world::World;

// Tests
#[cfg(test)]
mod camera_test;
#[cfg(test)]
mod material_test;
#[cfg(test)]
mod pattern_test;
#[cfg(test)]
mod ray_test;
#[cfg(test)]
mod shape_test;
#[cfg(test)]
mod world_test;
