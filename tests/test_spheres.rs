mod tests {
    use tracer::matrix::Matrix;
    use tracer::ray::Ray;
    use tracer::sphere::{intersect, normal_at, set_transform, Sphere};
    use tracer::transforms::{scaling, translation};
    use tracer::{point, vector};

    #[test]
    fn test_ray_intersect_sphere() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = intersect(s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn test_ray_tangent() {
        let r = Ray::new(point(0., 1., -5.), vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = intersect(s, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
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
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn test_object_intersection() {
        let r = Ray::new(point(0., 0., 5.), vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = intersect(s, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn test_default_transformation() {
        let s = Sphere::new();
        let identity = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);
        assert_eq!(s.transform, identity);
    }

    #[test]
    fn test_change_transform() {
        let mut s = Sphere::new();
        let t = translation(2., 3., 4.);
        set_transform(&mut s, t);
        assert_eq!(s.transform, t);
    }

    #[test]
    fn test_scaled_sphere_ray() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = Sphere::new();
        set_transform(&mut s, scaling(2., 2., 2.));
        let xs = intersect(s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.);
        assert_eq!(xs[1].t, 7.);
    }

    #[test]
    fn test_intersect_translated_sphere() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = Sphere::new();
        set_transform(&mut s, translation(5., 0., 0.));
        let xs = intersect(s, r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_normal() {
        let s = Sphere::new();
        let n = normal_at(s, point(1., 0., 0.));
        assert_eq!(n, vector(1., 0., 0.));
    }

    #[test]
    fn test_normal_y() {
        let s = Sphere::new();
        let n = normal_at(s, point(0., 1., 0.));
        assert_eq!(n, vector(0., 1., 0.));
    }
}
