use crate::{dot, materials::Material, normalize, reflect, Color, Tuple};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        PointLight {
            position,
            intensity,
        }
    }
}

pub fn lighting(
    material: Material,
    light: PointLight,
    point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
    in_shadow: bool,
) -> Color {
    let effective_color = material.color * light.intensity;
    let lightv = normalize(light.position - point);
    let ambient = effective_color * material.ambient;
    let black = Color::new(0., 0., 0.);

    let light_dot_normal = dot(lightv, normalv);
    let mut diffuse = black;
    let mut specular = black;

    if light_dot_normal >= 0. {
        diffuse = effective_color * material.diffuse * light_dot_normal;
        let reflectv = reflect(-lightv, normalv);
        let reflect_dot_eye = dot(reflectv, eyev);

        if reflect_dot_eye > 0. {
            let factor = f32::powf(reflect_dot_eye, material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    match in_shadow {
        true => ambient,
        _ => ambient + diffuse + specular,
    }
}
