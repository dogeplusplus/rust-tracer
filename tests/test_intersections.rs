mod tests {
    use tracer::intersections::{hit, prepare_computations, shlick, Intersection};
    use tracer::ray::Ray;
    use tracer::shape::Shape;
    use tracer::sphere::{glass_sphere, Sphere};
    use tracer::transforms::{scaling, translation};
    use tracer::world::ShapeEnum;
    use tracer::{point, vector};

    #[test]
    fn test_intersection() {
        let s = Sphere::default();
        let i = Intersection::new(3.5, ShapeEnum::Sphere(s));
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, ShapeEnum::Sphere(s));
    }

    #[test]
    fn test_intersections() {
        let s = Sphere::default();
        let i1 = Intersection::new(1., ShapeEnum::Sphere(s));
        let i2 = Intersection::new(2., ShapeEnum::Sphere(s));
        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[1].t, 2.);
    }

    #[test]
    fn test_hit_positive() {
        let s = Sphere::default();
        let i1 = Intersection::new(1., ShapeEnum::Sphere(s));
        let i2 = Intersection::new(2., ShapeEnum::Sphere(s));
        let xs = vec![i1.clone(), i2];
        let i = hit(xs);
        assert_eq!(i.unwrap(), i1);
    }

    #[test]
    fn test_hit_some_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1., ShapeEnum::Sphere(s));
        let i2 = Intersection::new(1., ShapeEnum::Sphere(s));
        let xs = vec![i1, i2.clone()];
        let i = hit(xs);
        assert_eq!(i.unwrap(), i2);
    }

    #[test]
    fn test_all_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2., ShapeEnum::Sphere(s));
        let i2 = Intersection::new(-1., ShapeEnum::Sphere(s));
        let xs = vec![i1, i2];
        let i = hit(xs);
        assert!(i.is_none());
    }

    #[test]
    fn test_first_hit_nonnegative() {
        let s = Sphere::default();
        let i1 = Intersection::new(5., ShapeEnum::Sphere(s));
        let i2 = Intersection::new(7., ShapeEnum::Sphere(s));
        let i3 = Intersection::new(-3., ShapeEnum::Sphere(s));
        let i4 = Intersection::new(2., ShapeEnum::Sphere(s));
        let xs = vec![i1, i2, i3, i4.clone()];
        let i = hit(xs);
        assert_eq!(i.unwrap(), i4);
    }

    #[test]
    fn test_precompute_intersection_state() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let shape = Sphere::default();
        let i = Intersection::new(4., ShapeEnum::Sphere(shape));
        let comps = prepare_computations(i.clone(), r, vec![i.clone()]);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, point(0., 0., -1.));
        assert_eq!(comps.eyev, vector(0., 0., -1.));
        assert_eq!(comps.normalv, vector(0., 0., -1.));
    }

    #[test]
    fn test_hit_outside() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let shape = Sphere::default();
        let i = Intersection::new(4., ShapeEnum::Sphere(shape));
        let comps = prepare_computations(i.clone(), r, vec![i]);
        assert!(!comps.inside);
    }

    #[test]
    fn test_hit_inside() {
        let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let shape = Sphere::default();
        let i = Intersection::new(1., ShapeEnum::Sphere(shape));
        let comps = prepare_computations(i.clone(), r, vec![i]);
        assert_eq!(comps.point, point(0., 0., 1.));
        assert_eq!(comps.eyev, vector(0., 0., -1.));
        assert!(comps.inside);
        assert_eq!(comps.normalv, vector(0., 0., -1.));
    }

    #[test]
    fn test_refraction_multi_intersection() {
        let mut a = glass_sphere();
        a.set_transform(scaling(2., 2., 2.));
        a.material.refractive_index = 1.5;

        let mut b = glass_sphere();
        b.set_transform(translation(0., 0., -0.25));
        b.material.refractive_index = 2.0;

        let mut c = glass_sphere();
        c.set_transform(translation(0., 0., 0.25));
        c.material.refractive_index = 2.5;

        let xs = vec![
            Intersection::new(2., ShapeEnum::Sphere(a)),
            Intersection::new(2.75, ShapeEnum::Sphere(b)),
            Intersection::new(3.25, ShapeEnum::Sphere(c)),
            Intersection::new(4.75, ShapeEnum::Sphere(b)),
            Intersection::new(5.25, ShapeEnum::Sphere(c)),
            Intersection::new(6., ShapeEnum::Sphere(a)),
        ];

        let results = vec![
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];

        let r = Ray::new(point(0., 0., -4.), vector(0., 0., 1.));
        for idx in 0..results.len() {
            let intersection = xs[idx].clone();
            let result = results[idx];
            let comps = prepare_computations(intersection, r, xs.clone());
            assert_eq!(comps.n1, result.0);
            assert_eq!(comps.n2, result.1);
        }
    }

    #[test]
    fn test_under_point_below_surface() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let mut shape = glass_sphere();
        shape.transform = translation(0., 0., 1.);
        let i = Intersection::new(5., ShapeEnum::Sphere(shape));
        let xs = vec![i.clone()];
        let comps = prepare_computations(i, r, xs);
        assert_eq!(comps.under_point.z, 1e-4);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn test_shlick_approximation() {
        let shape = glass_sphere();
        let r = Ray::new(point(0., 0., f32::sqrt(2.) / 2.), vector(0., 1., 0.));
        let xs = vec![
            Intersection::new(-f32::sqrt(2.) / 2., ShapeEnum::Sphere(shape)),
            Intersection::new(f32::sqrt(2.) / 2., ShapeEnum::Sphere(shape)),
        ];
        let comps = prepare_computations(xs[1].clone(), r, xs);
        let reflectance = shlick(&comps);
        assert_eq!(reflectance, 1.);
    }

    #[test]
    fn test_shlick_perpendicular() {
        let shape = ShapeEnum::Sphere(glass_sphere());
        let r = Ray::new(point(0., 0., 0.), vector(0., 1., 0.));
        let xs = vec![
            Intersection::new(-1., shape.clone()),
            Intersection::new(1., shape),
        ];
        let comps = prepare_computations(xs[1].clone(), r, xs);
        let reflectance = shlick(&comps);
        assert_eq!(reflectance, 0.040000003);
    }

    #[test]
    fn test_shlick_small_angle() {
        let shape = ShapeEnum::Sphere(glass_sphere());
        let r = Ray::new(point(0., 0.99, -2.), vector(0., 0., 1.));
        let xs = vec![Intersection::new(1.8589, shape)];
        let comps = prepare_computations(xs[0].clone(), r, xs);
        let reflectance = shlick(&comps);
        assert_eq!(reflectance, 0.48873067);
    }
}
