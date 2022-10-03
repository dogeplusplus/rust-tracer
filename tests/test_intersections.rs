mod tests {
    use tracer::intersections::{hit, prepare_computations, Intersection};
    use tracer::ray::Ray;
    use tracer::sphere::Sphere;
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
        let xs = vec![i1, i2];
        let i = hit(xs);
        assert_eq!(i.unwrap(), i1);
    }

    #[test]
    fn test_hit_some_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1., ShapeEnum::Sphere(s));
        let i2 = Intersection::new(1., ShapeEnum::Sphere(s));
        let xs = vec![i1, i2];
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
        let xs = vec![i1, i2, i3, i4];
        let i = hit(xs);
        assert_eq!(i.unwrap(), i4);
    }

    #[test]
    fn test_precompute_intersection_state() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let shape = Sphere::default();
        let i = Intersection::new(4., ShapeEnum::Sphere(shape));
        let comps = prepare_computations(i, r);
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
        let comps = prepare_computations(i, r);
        assert!(!comps.inside);
    }

    #[test]
    fn test_hit_inside() {
        let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let shape = Sphere::default();
        let i = Intersection::new(1., ShapeEnum::Sphere(shape));
        let comps = prepare_computations(i, r);
        assert_eq!(comps.point, point(0., 0., 1.));
        assert_eq!(comps.eyev, vector(0., 0., -1.));
        assert!(comps.inside);
        assert_eq!(comps.normalv, vector(0., 0., -1.));
    }
}
