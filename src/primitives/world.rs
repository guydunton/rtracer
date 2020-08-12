use super::{Intersection, IntersectionStats, Material, PointLight, Ray, Sphere};
use crate::{
    canvas::Color,
    maths::{Matrix4x4, Point},
};

pub struct WorldImpl {
    lights: Vec<PointLight>,
    objects: Vec<Sphere>,
}

impl WorldImpl {
    pub fn lights(&self) -> Vec<PointLight> {
        self.lights.clone()
    }

    pub fn objects(&self) -> Vec<Sphere> {
        self.objects.clone()
    }

    pub fn ray_intersects(&self, ray: Ray) -> Vec<Intersection> {
        let mut f: Vec<Intersection> = self
            .objects
            .iter()
            .map(|obj| ray.intersects(*obj))
            .flatten()
            .collect();
        f.sort_unstable();
        f
    }

    pub fn shade_hit(&self, comps: IntersectionStats) -> Color {
        self.lights
            .iter()
            .map(|light| {
                comps.object().material().lighting(
                    *light,
                    comps.point(),
                    comps.eyev(),
                    comps.normalv(),
                )
            })
            .fold(Color::black(), |total, col| total + col)
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let intersections = self.ray_intersects(ray.clone());

        let hit = Intersection::hit(intersections);

        match hit {
            Some(intersection) => {
                let comps = intersection.prepare_computations(ray);
                self.shade_hit(comps)
            }
            None => Color::black(),
        }
    }
}

/// World is a builder for WorldImpl
pub struct World {
    lights: Vec<PointLight>,
    objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> World {
        World {
            lights: vec![],
            objects: vec![],
        }
    }

    pub fn generate(self) -> WorldImpl {
        WorldImpl {
            lights: self.lights,
            objects: self.objects,
        }
    }

    pub fn reset_lights(mut self) -> Self {
        self.lights = vec![];
        self
    }

    pub fn add_light(mut self, light: PointLight) -> Self {
        self.lights.push(light);
        self
    }

    pub fn add_object(mut self, object: Sphere) -> Self {
        self.objects.push(object);
        self
    }

    pub fn default() -> World {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white());

        let mut material = Material::default();
        material.color = Color::new(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;

        let sphere1 = Sphere::new(Matrix4x4::identity(), material);
        let sphere2 = Sphere::new(Matrix4x4::scaling(0.5, 0.5, 0.5), Material::default());

        World::new()
            .add_light(light)
            .add_object(sphere1)
            .add_object(sphere2)
    }
}
