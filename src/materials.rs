use crate::{patterns::Pattern, Color};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflective: f32,
    pub refractive_index: f32,
    pub transparency: f32,
    pub pattern: Option<Pattern>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1., 1., 1.),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
            reflective: 0.,
            transparency: 0.,
            refractive_index: 1.,
            pattern: None,
        }
    }
}
