mod tests {
    use tracer::matrix::Matrix;
    use tracer::patterns::{
        pattern_at_shape, CheckerPattern, GradientPattern, Pattern, PatternType, RingPattern,
        StripePattern, RadialGradient,
    };
    use tracer::shape::Shape;
    use tracer::sphere::Sphere;
    use tracer::transforms::{scaling, translation};
    use tracer::world::ShapeEnum;
    use tracer::{point, Color};

    #[test]
    fn test_make_stripe_pattern() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = StripePattern::new(white, black);

        assert_eq!(pattern.a, white);
        assert_eq!(pattern.b, black);
    }

    #[test]
    fn test_default_pattern() {
        let identity = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);
        let pattern = Pattern::new(PatternType::Test());
        assert_eq!(pattern.transform, identity);
    }

    #[test]
    fn test_assign_transform() {
        let mut pattern = Pattern::new(PatternType::Test());
        let trans = translation(1., 2., 3.);
        pattern.set_transform(trans);
        assert_eq!(pattern.transform, trans);
    }

    #[test]
    fn test_stripe_constant_y() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = StripePattern::new(white, black);

        assert_eq!(pattern.local_pattern_at(point(0., 0., 0.)), white);
        assert_eq!(pattern.local_pattern_at(point(0., 1., 0.)), white);
        assert_eq!(pattern.local_pattern_at(point(0., 2., 0.)), white);
    }

    #[test]
    fn test_stripe_constant_z() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = StripePattern::new(white, black);

        assert_eq!(pattern.local_pattern_at(point(0., 0., 0.)), white);
        assert_eq!(pattern.local_pattern_at(point(0., 0., 1.)), white);
        assert_eq!(pattern.local_pattern_at(point(0., 0., 2.)), white);
    }

    #[test]
    fn test_stripe_alternates_x() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = StripePattern::new(white, black);

        assert_eq!(pattern.local_pattern_at(point(0., 0., 0.)), white);
        assert_eq!(pattern.local_pattern_at(point(0.9, 0., 0.)), white);
        assert_eq!(pattern.local_pattern_at(point(1., 0., 0.)), black);
        assert_eq!(pattern.local_pattern_at(point(-0.1, 0., 0.)), black);
        assert_eq!(pattern.local_pattern_at(point(-1., 0., 0.)), black);
        assert_eq!(pattern.local_pattern_at(point(-1.1, 0., 0.)), white);
    }

    #[test]
    fn test_pattern_obj_transform() {
        let mut object = Sphere::default();
        object.set_transform(scaling(2., 2., 2.));
        let object = ShapeEnum::Sphere(object);
        let pattern = Pattern::new(PatternType::Test());
        let c = pattern_at_shape(pattern, object, point(2., 3., 4.));
        assert_eq!(c, Color::new(1., 1.5, 2.));
    }

    #[test]
    fn test_pattern_transform() {
        let object = ShapeEnum::Sphere(Sphere::default());
        let mut pattern = Pattern::new(PatternType::Test());
        pattern.set_transform(scaling(2., 2., 2.));
        let c = pattern_at_shape(pattern, object, point(2., 3., 4.));
        assert_eq!(c, Color::new(1., 1.5, 2.));
    }

    #[test]
    fn test_stripe_obj_pattern_transform() {
        let mut object = Sphere::default();
        object.set_transform(scaling(2., 2., 2.));
        let object = ShapeEnum::Sphere(object);
        let mut pattern = Pattern::new(PatternType::Test());
        pattern.set_transform(translation(0.5, 1., 1.5));
        let c = pattern_at_shape(pattern, object, point(2.5, 3., 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }

    #[test]
    fn test_gradient_pattern() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = GradientPattern::new(white, black);

        assert_eq!(pattern.local_pattern_at(point(0., 0., 0.)), white);
        assert_eq!(
            pattern.local_pattern_at(point(0.25, 0., 0.)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.local_pattern_at(point(0.5, 0., 0.)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.local_pattern_at(point(0.75, 0., 0.)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn test_ring_pattern() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = RingPattern::new(white, black);

        assert_eq!(pattern.local_pattern_at(point(0., 0., 0.)), white);
        assert_eq!(pattern.local_pattern_at(point(1., 0., 0.)), black);
        assert_eq!(pattern.local_pattern_at(point(0., 0., 1.)), black);
        assert_eq!(pattern.local_pattern_at(point(0.708, 0., 0.708)), black);
    }

    #[test]
    fn test_checker_pattern() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = CheckerPattern::new(white, black);

        // test repeat in x, y and z
        assert_eq!(pattern.local_pattern_at(point(0., 0., 0.)), white);
        assert_eq!(pattern.local_pattern_at(point(0.99, 0., 0.)), white);
        assert_eq!(pattern.local_pattern_at(point(1.01, 0., 0.)), black);

        assert_eq!(pattern.local_pattern_at(point(0., 0., 0.)), white);
        assert_eq!(pattern.local_pattern_at(point(0., 0.99, 0.)), white);
        assert_eq!(pattern.local_pattern_at(point(0., 1.01, 0.)), black);

        assert_eq!(pattern.local_pattern_at(point(0., 0., 0.)), white);
        assert_eq!(pattern.local_pattern_at(point(0., 0., 0.99)), white);
        assert_eq!(pattern.local_pattern_at(point(0., 0., 1.01)), black);
    }

    #[test]
    fn test_radial_gradient_pattern() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = RadialGradient::new(white, black);

        assert_eq!(pattern.local_pattern_at(point(1., 0., 0.)), white);
        assert_eq!(pattern.local_pattern_at(point(0.5, 0., 0.)),
            Color::new(0.5, 0.5, 0.5));
    }
}
