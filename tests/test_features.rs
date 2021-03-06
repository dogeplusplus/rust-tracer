mod tests {
    use tracer::{cross, dot, magnitude, normalize, point, reflect, vector, Color, Tuple};

    #[test]
    fn test_vector_raw() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);

        assert!(a.is_vector());
        assert_eq!(a.is_point(), false);
    }

    #[test]
    fn test_point_raw() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.);

        assert!(a.is_point());
        assert_eq!(a.is_vector(), false);
    }

    #[test]
    fn test_point() {
        let p: Tuple = point(4., -4., 3.);
        assert_eq!(p, Tuple::new(4., -4., 3., 1.));
    }

    #[test]
    fn test_vector() {
        let p: Tuple = vector(4., -4., 3.);
        assert_eq!(p, Tuple::new(4., -4., 3., 0.));
    }

    #[test]
    fn test_add_tuple() {
        let a1 = Tuple::new(3., -2., 5., 1.);
        let a2: Tuple = Tuple::new(-2., 3.0, 1., 0.);

        let sum = a1 + a2;
        assert_eq!(sum, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn test_sub_points() {
        let p1 = point(3., 2., 1.);
        let p2 = point(5., 6., 7.);

        let diff = p1 - p2;
        assert_eq!(diff, vector(-2., -4., -6.));
    }

    #[test]
    fn test_sub_vec_from_point() {
        let p = point(3., 2., 1.);
        let v = vector(5., 6., 7.);

        let diff = p - v;
        assert_eq!(diff, point(-2., -4., -6.));
    }

    #[test]
    fn test_sub_vec() {
        let v1 = vector(3., 2., 1.);
        let v2 = vector(5., 6., 7.);
        let diff = v1 - v2;

        assert_eq!(diff, vector(-2., -4., -6.));
    }

    #[test]
    fn test_sub_zero() {
        let zero = vector(0., 0., 0.);
        let v = vector(1., -2., -3.);
        let diff = zero - v;

        assert_eq!(diff, vector(-1., 2., 3.));
    }

    #[test]
    fn test_negate() {
        let a = Tuple::new(1., -2., 3., -4.);
        assert_eq!(-a, Tuple::new(-1., 2., -3., 4.));
    }

    #[test]
    fn test_multiply() {
        let a = Tuple::new(1., -2., 3., -4.);
        let result = a * 3.5;
        assert_eq!(result, Tuple::new(3.5, -7., 10.5, -14.));
    }

    #[test]
    fn test_multiply_fraction() {
        let a = Tuple::new(1., -2., 3., -4.);
        let result = a * 0.5;
        assert_eq!(result, Tuple::new(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn test_divide() {
        let a = Tuple::new(1., -2., 3., -4.);
        let result = a / 2.;
        assert_eq!(result, Tuple::new(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn test_magnitude() {
        let v = vector(1., 0., 0.);
        assert_eq!(magnitude(v), 1.);

        let v = vector(0., 1., 0.);
        assert_eq!(magnitude(v), 1.);

        let v = vector(0., 0., 1.);
        assert_eq!(magnitude(v), 1.);

        let v = vector(1., 2., 3.);
        assert_eq!(magnitude(v), f32::sqrt(14.));

        let v = vector(-1., -2., -3.);
        assert_eq!(magnitude(v), f32::sqrt(14.));
    }

    #[test]
    fn test_normalize() {
        let v = vector(4., 0., 0.);
        assert_eq!(normalize(v), vector(1., 0., 0.));

        let v = vector(1., 2., 3.);
        assert_eq!(
            normalize(v),
            vector(
                1. / f32::sqrt(14.),
                2. / f32::sqrt(14.),
                3. / f32::sqrt(14.)
            )
        );

        let norm = normalize(v);
        assert!((magnitude(norm) - 1.).abs() < f32::EPSILON);
    }

    #[test]
    fn test_dot_product() {
        let a = vector(1., 2., 3.);
        let b = vector(2., 3., 4.);
        assert_eq!(dot(a, b), 20.);
    }

    #[test]
    fn test_cross_product() {
        let a = vector(1., 2., 3.);
        let b = vector(2., 3., 4.);

        assert_eq!(cross(a, b), vector(-1., 2., -1.));
        assert_eq!(cross(b, a), vector(1., -2., 1.))
    }

    #[test]
    fn test_color() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn test_add_color() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_sub_color() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn test_mul_color_scalar() {
        let c1 = Color::new(0.2, 0.3, 0.4);
        assert_eq!(c1 * 2., Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn test_mul_color() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        let expected = Color::new(0.9, 0.2, 0.04);
        assert_eq!(c1 * c2, expected);
    }

    #[test]
    fn test_reflect_45() {
        let v = vector(1., -1., 0.);
        let n = vector(0., 1., 0.);
        let r = reflect(v, n);
        assert_eq!(r, vector(1., 1., 0.));
    }

    #[test]
    fn test_reflect_slanted() {
        let v = vector(0., -1., 0.);
        let n = vector(f32::sqrt(2.) / 2., f32::sqrt(2.) / 2., 0.);
        let r = reflect(v, n);
        let diff = r - vector(1., 0., 0.);
        assert!(magnitude(diff) < f32::EPSILON);
    }
}
