mod tests {
    use tracer::{point,vector};
    use tracer::ray::{Ray,position,transform};
    use tracer::transforms::{translation,scaling};

    #[test]
    fn test_query_ray() {
        let origin = point(1., 2., 3.);
        let direction = vector(4., 5., 6.);

        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn test_compute_point_from_distance() {
        let r = Ray::new(point(2., 3., 4.), vector(1., 0., 0.));

        assert_eq!(position(r, 0.), point(2., 3., 4.));
        assert_eq!(position(r, 1.), point(3., 3., 4.));
        assert_eq!(position(r, -1.), point(1., 3., 4.));
        assert_eq!(position(r, 2.5), point(4.5, 3., 4.));
        
    }

    #[test]
    fn test_translate_ray() {
        let r = Ray::new(point(1., 2., 3.), vector(0., 1., 0.));
        let m = translation(3., 4., 5.);
        let r2 = transform(r, m);
        
        assert_eq!(r2.origin, point(4., 6., 8.));
        assert_eq!(r2.direction, vector(0., 1., 0.));
    }

    #[test]
    fn test_scale_ray() {
        let r = Ray::new(point(1., 2., 3.), vector(0., 1., 0.));
        let m = scaling(2., 3., 4.);
        let r2 = transform(r, m);
        
        assert_eq!(r2.origin, point(2., 6., 12.));
        assert_eq!(r2.direction, vector(0., 3., 0.));
    }
}