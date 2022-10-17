mod tests {
    use tracer::shape::Shape;
    use tracer::cube::Cube;
    use tracer::{ray::Ray, vector, point};
    use std::iter::zip;

    #[test]
    fn test_ray_intersect_cube() {
        let c = Cube::default();

        let rays = vec![
            Ray::new(point(5., 0.5, 0.), vector(-1., 0., 0.)),
            Ray::new(point(-5., 0.5, 0.), vector(1., 0., 0.)),
            Ray::new(point(0.5, 5., 0.), vector(0., -1., 0.)),
            Ray::new(point(0.5, -5., 0.), vector(0., 1., 0.)),
            Ray::new(point(0.5, 0., 5.), vector(0., 0., -1.)),
            Ray::new(point(0.5, 0., -5.), vector(0., 0., 1.)),
            Ray::new(point(0., 0.5, 0.), vector(0., 0., 1.)),
        ];
        let ts = vec![
            [4., 6.],
            [4., 6.],
            [4., 6.],
            [4., 6.],
            [4., 6.],
            [4., 6.],
            [-1., 1.],
        ];

        for (r, ts) in zip(rays, ts) {
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, ts[0]);
            assert_eq!(xs[1].t, ts[1]);
        }
    }

    #[test]
    fn test_ray_misses_cube() {
        let c = Cube::default();

        let rays = vec![
            Ray::new(point(-2., 0. ,0.), vector(0.2673, 0.5345, 0.8018)),
            Ray::new(point(0., -2., 0.), vector(0.8018, 0.2673, 0.5345)),
            Ray::new(point(0., 0., -2.), vector(0.5345, 0.8018, 0.2673)),
            Ray::new(point(2., 0., 2.), vector(0., 0., -1.)),
            Ray::new(point(0., 2., 2.), vector(0., -1., 0.)),
            Ray::new(point(2., 2., 0.), vector(-1., 0., 0.)),
        ];

        for r in rays {
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), 0);
        }
    }
}