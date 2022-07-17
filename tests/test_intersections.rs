mod tests {
    use tracer::intersections::{hit, Intersection};
    use tracer::sphere::Sphere;

    #[test]
    fn test_intersection() {
        let s = Sphere::default();
        let i = Intersection::new(3.5, s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn test_intersections() {
        let s = Sphere::default();
        let i1 = Intersection::new(1., s);
        let i2 = Intersection::new(2., s);
        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[1].t, 2.);
    }

    #[test]
    fn test_hit_positive() {
        let s = Sphere::default();
        let i1 = Intersection::new(1., s);
        let i2 = Intersection::new(2., s);
        let xs = vec![i1, i2];
        let i = hit(xs);
        assert_eq!(i.unwrap(), i1);
    }

    #[test]
    fn test_hit_some_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1., s);
        let i2 = Intersection::new(1., s);
        let xs = vec![i1, i2];
        let i = hit(xs);
        assert_eq!(i.unwrap(), i2);
    }

    #[test]
    fn test_all_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2., s);
        let i2 = Intersection::new(-1., s);
        let xs = vec![i1, i2];
        let i = hit(xs);
        assert!(i.is_none());
    }

    #[test]
    fn test_first_hit_nonnegative() {
        let s = Sphere::default();
        let i1 = Intersection::new(5., s);
        let i2 = Intersection::new(7., s);
        let i3 = Intersection::new(-3., s);
        let i4 = Intersection::new(2., s);
        let xs = vec![i1, i2, i3, i4];
        let i = hit(xs);
        assert_eq!(i.unwrap(), i4);
    }
}
