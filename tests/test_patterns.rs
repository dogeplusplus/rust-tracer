mod tests {
    use tracer::shape::Shape;
    use tracer::sphere::Sphere;
    use tracer::transforms::{scaling, translation};
    use tracer::world::ShapeEnum;
    use tracer::{Color, point};
    use tracer::patterns::{stripe_pattern,stripe_at,stripe_at_object};

    #[test]
    fn test_make_stripe_pattern() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = stripe_pattern(white, black);

        assert_eq!(pattern.a, white);
        assert_eq!(pattern.b, black);
    }

    #[test]
    fn test_stripe_constant_y() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = stripe_pattern(white, black);
        
        assert_eq!(stripe_at(pattern, point(0., 0., 0.)), white);
        assert_eq!(stripe_at(pattern, point(0., 1., 0.)), white);
        assert_eq!(stripe_at(pattern, point(0., 2., 0.)), white);
    }

    #[test]
    fn test_stripe_constant_z() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = stripe_pattern(white, black);
        
        assert_eq!(stripe_at(pattern, point(0., 0., 0.)), white);
        assert_eq!(stripe_at(pattern, point(0., 0., 1.)), white);
        assert_eq!(stripe_at(pattern, point(0., 0., 2.)), white);
    }

    #[test]
    fn test_stripe_alternates_x() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = stripe_pattern(white, black);

        assert_eq!(stripe_at(pattern, point(0., 0., 0.)), white);
        assert_eq!(stripe_at(pattern, point(0.9, 0., 0.)), white);
        assert_eq!(stripe_at(pattern, point(1., 0., 0.)), black);
        assert_eq!(stripe_at(pattern, point(-0.1, 0., 0.)), black);
        assert_eq!(stripe_at(pattern, point(-1., 0., 0.)), black);
        assert_eq!(stripe_at(pattern, point(-1.1, 0., 0.)), white);
    }

    #[test]
    fn test_stripes_obj_transform() {
        let mut object = Sphere::default();
        object.set_transform(scaling(2., 2., 2.));
        let object = ShapeEnum::Sphere(object);
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = stripe_pattern(white, black);
        let c = stripe_at_object(pattern, object, point(1.5, 0., 0.));
        assert_eq!(c, white);
    }

    #[test]
    fn test_stripes_pattern_transform() {
        let object = ShapeEnum::Sphere(Sphere::default());
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let mut pattern = stripe_pattern(white, black);
        pattern.set_transform(scaling(2., 2., 2.));
        let c = stripe_at_object(pattern, object, point(1.5, 0., 0.));
        assert_eq!(c, white);
    }

    #[test]
    fn test_stripe_obj_pattern_transform() {
        let mut object = Sphere::default();
        object.set_transform(scaling(2., 2., 2.));
        let object = ShapeEnum::Sphere(object);
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let mut pattern = stripe_pattern(white, black);
        pattern.set_transform(translation(0.5, 0., 0.));
        let c = stripe_at_object(pattern, object, point(2.5, 0., 0.));
        assert_eq!(c, white);
    }
}