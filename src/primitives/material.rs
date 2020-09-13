use super::PointLight;
use crate::{
    image::Color,
    maths::{Point, Vector},
};

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn default() -> Self {
        Self {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn lighting(
        &self,
        light: PointLight,
        position: Point,
        eyev: Vector,
        normalv: Vector,
        in_shadow: bool,
    ) -> Color {
        // Combine the surface color with the light's color/intensity
        let effective_color = self.color * light.intensity();

        // Find the direction to the light source
        let lightv = (light.position() - position).normalize();

        // Compute the ambient contribution
        let ambient = effective_color * self.ambient;

        /*
            light_dot_normal represents the cosine of the angle between the light
            vector and the normal vector. A negative number means the light is on
            the other side of the surface
        */
        let light_dot_normal = Vector::dot(lightv, normalv);
        let diffuse: Color;
        let specular: Color;

        if light_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            // Compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            /*
                reflect_dot_eye represents the cosine of the angle between the reflection
                vector and the eye vector. A negative number means the light reflects away
                from the eye.
            */
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = Vector::dot(reflectv, eyev);

            if reflect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                // Compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity() * self.specular * factor;
            }
        }

        if in_shadow {
            // In shadow so only show ambient
            ambient
        } else {
            // Add the three contributions together to get the final shading
            ambient + diffuse + specular
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        use crate::maths::is_same;
        self.color == other.color
            && is_same(self.ambient, other.ambient)
            && is_same(self.diffuse, other.diffuse)
            && is_same(self.specular, other.specular)
            && is_same(self.shininess, other.shininess)
    }
}
