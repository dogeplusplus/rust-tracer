mod tests {
    use tracer::sphere::Sphere;
    use tracer::intersections::Intersection;

    #[test]
    fn test_intersection() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn test_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1., s);
        let i2 = Intersection::new(2., s);
        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[1].t, 2.);
    }

}