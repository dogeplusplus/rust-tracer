mod tests {
    use tracer::lights::{lighting, PointLight};
    use tracer::materials::Material;
    use tracer::{point, vector, Color};

    #[test]
    fn test_materials() {
        let m = Material::default();

        assert_eq!(m.color, Color::new(1., 1., 1.));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.);
    }

    #[test]
    fn test_lighting_eye_between_surface_and_source() {
        let m = Material::default();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(point(0., 0., -10.), Color::new(1., 1., 1.));
        let result = lighting(m, light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn test_lighting_eye_angle_45_surface() {
        let m = Material::default();
        let position = point(0., 0., 0.);
        let eyev = vector(0., f32::sqrt(2.) / 2., f32::sqrt(2.) / 2.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(point(0., 0., -10.), Color::new(1., 1., 1.));
        let result = lighting(m, light, position, eyev, normalv);
        assert_eq!(result, Color::new(1., 1., 1.));
    }

    #[test]
    fn test_lighting_angle_45_surface() {
        let m = Material::default();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(point(0., 10., -10.), Color::new(1., 1., 1.));
        let result = lighting(m, light, position, eyev, normalv);
        let c = 0.1 + 0.9 * f32::sqrt(2.) / 2.;
        assert_eq!(result, Color::new(c, c, c));
    }

    #[test]
    fn test_eye_path_reflection_vector() {
        let m = Material::default();
        let position = point(0., 0., 0.);
        let eyev = vector(0., -f32::sqrt(2.) / 2., -f32::sqrt(2.) / 2.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(point(0., 10., -10.), Color::new(1., 1., 1.));
        let result = lighting(m, light, position, eyev, normalv);
        let c = 0.1 + 0.9 * f32::sqrt(2.) / 2. + 0.9;

        // Precision seems to be quite low for the lighting calculation
        assert!((c - result.red).abs() < 1e-4);
        assert!((c - result.green).abs() < 1e-4);
        assert!((c - result.blue).abs() < 1e-4);
    }

    #[test]
    fn test_light_behind_surface() {
        let m = Material::default();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(point(0., 0., 10.), Color::new(1., 1., 1.));
        let result = lighting(m, light, position, eyev, normalv);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
