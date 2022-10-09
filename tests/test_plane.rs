mod tests {
    use tracer::shape::Shape;
    use tracer::world::ShapeEnum;
    use tracer::{plane::Plane, point, ray::Ray, vector};

    #[test]
    fn test_normal_plane_constant() {
        let p = Plane::default();
        let n1 = p.local_normal_at(point(0., 0., 0.));
        let n2 = p.local_normal_at(point(10., 0., -10.));
        let n3 = p.local_normal_at(point(-5., 0., 150.));

        assert_eq!(n1, vector(0., 1., 0.));
        assert_eq!(n2, vector(0., 1., 0.));
        assert_eq!(n3, vector(0., 1., 0.));
    }

    #[test]
    fn test_intersect_ray_plane() {
        let p = Plane::default();
        let r = Ray::new(point(0., 10., 0.), vector(0., 0., 1.));
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_intersect_coplanar_plane() {
        let p = Plane::default();
        let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_above_plane_intersection() {
        let p = Plane::default();
        let r = Ray::new(point(0., 1., 0.), vector(0., -1., 0.));
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[0].object, ShapeEnum::Plane(p));
    }

    #[test]
    fn test_below_plane_intersection() {
        let p = Plane::default();
        let r = Ray::new(point(0., -1., 0.), vector(0., 1., 0.));
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[0].object, ShapeEnum::Plane(p));
    }
}
