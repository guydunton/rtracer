// Code
mod camera;
mod intersection;
mod intersection_stats;
mod material;
mod point_light;
mod ray;
mod shape;
mod world;

// Exports
pub use camera::Camera;
pub use intersection::Intersection;
pub use intersection_stats::IntersectionStats;
pub use material::Material;
pub use point_light::PointLight;
pub use ray::Ray;
pub use shape::Sphere;
pub use world::World;

// Tests
#[cfg(test)]
mod material_test;
#[cfg(test)]
mod ray_test;
#[cfg(test)]
mod shape_test;
#[cfg(test)]
mod world_test;
