mod tests {
    use tracer::{lights::PointLight, point, Color};

    #[test]
    fn test_point_light() {
        let intensity = Color::new(1., 1., 1.);
        let position = point(0., 0., 0.);
        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
