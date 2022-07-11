mod tests {
    use tracer::{point,vector};
    use tracer::ray::Ray;
    use tracer::sphere::{intersect, Sphere};

    #[test]
    fn test_ray_intersect_sphere() {
            let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
            let s = Sphere::new();
            let xs = intersect(s, r);

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0], 4.0);
            assert_eq!(xs[1], 6.0)
    }

    #[test]
    fn test_ray_tangent() {
        let r = Ray::new(point(0., 1., -5.), vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = intersect(s, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }

    #[test]
    fn test_non_intersect() {
        let r = Ray::new(point(0., 2., -5.), vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = intersect(s, r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_sphere_behind_ray() {
        let r = Ray::new(point(0., 0., 5.), vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = intersect(s, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0)
    }
}