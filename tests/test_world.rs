mod tests {
    use tracer::{
        lights::PointLight,
        materials::Material,
        point,
        ray::Ray,
        sphere::Sphere,
        transforms::scaling,
        vector,
        world::{contains, intersect_world, World},
        Color,
    };

    #[test]
    fn test_world() {
        let w = World::new();
        assert_eq!(w.objects.len(), 0);
        assert!(w.light.is_none());
    }

    #[test]
    fn test_default_world() {
        let light = PointLight::new(point(-10., 10., -10.), Color::new(1., 1., 1.));
        let mut s1 = Sphere::default();
        let mut m = Material::default();
        m.color = Color::new(0.8, 1.0, 0.6);
        m.diffuse = 0.7;
        m.specular = 0.2;
        s1.material = m;

        let mut s2 = Sphere::default();
        s2.transform = scaling(0.5, 0.5, 0.5);

        let w = World::default();
        assert_eq!(w.light.unwrap(), light);
        assert!(contains(&w, s1));
        assert!(contains(&w, s2));
    }

    #[test]
    fn test_intersect_world_ray() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let xs = intersect_world(w, r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.);
    }
}
