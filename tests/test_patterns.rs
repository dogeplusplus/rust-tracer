mod tests {
    use tracer::{Color, point};
    use tracer::patterns::{stripe_pattern,stripe_at};

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
}