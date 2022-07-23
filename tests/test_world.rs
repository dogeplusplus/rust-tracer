mod tests {
    use tracer::{
        intersections::prepare_computations,
        intersections::Intersection,
        lights::PointLight,
        materials::Material,
        point,
        ray::Ray,
        sphere::Sphere,
        transforms::scaling,
        vector,
        world::{contains, intersect_world, World, shade_hit, color_at},
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
        let xs = intersect_world(&w, r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.);
    }

    #[test]
    fn test_shade_intersection() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let shape = w.objects[0];
        let i = Intersection::new(4., shape);
        let comps = prepare_computations(i, r);
        let c = shade_hit(&w, comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn test_shade_intersection_inside() {
        let mut w = World::default();
        w.light = Some(PointLight::new(point(0., 0.25, 0.), Color::new(1., 1., 1.)));
        let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let shape = w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = prepare_computations(i, r);
        let c = shade_hit(&w, comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn test_intersect_world_ray_miss() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 1., 0.));
        let c = color_at(&w, r);
        assert_eq!(c, Color::new(0., 0., 0.));
    }

    #[test]
    fn test_intersect_world_ray_hit() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let c = color_at(&w, r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_intersection_behind_ray() {
        let mut w = World::default();

        w.objects[0].material.ambient = 1.;
        w.objects[1].material.ambient = 1.;
        let r = Ray::new(point(0., 0., 0.75), vector(0., 0., -1.));
        let c = color_at(&w, r);
        assert_eq!(c, w.objects[1].material.color);
    }
}