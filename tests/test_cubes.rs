mod tests {
    use std::iter::zip;
    use tracer::cube::Cube;
    use tracer::shape::Shape;
    use tracer::{point, ray::Ray, vector};

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
            Ray::new(point(-2., 0., 0.), vector(0.2673, 0.5345, 0.8018)),
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

    #[test]
    fn test_normal_cube() {
        let c = Cube::default();

        let points = vec![
            point(1., 0.5, -0.8),
            point(-1., -0.2, 0.9),
            point(-0.4, 1., -0.1),
            point(0.3, -1., -0.7),
            point(-0.6, 0.3, 1.),
            point(0.4, 0.4, -1.),
            point(1., 1., 1.),
            point(-1., -1., -1.),
        ];

        let normals = vec![
            vector(1., 0., 0.),
            vector(-1., 0., 0.),
            vector(0., 1., 0.),
            vector(0., -1., 0.),
            vector(0., 0., 1.),
            vector(0., 0., -1.),
            vector(1., 0., 0.),
            vector(-1., 0., 0.),
        ];

        for (p, n) in zip(points, normals) {
            let normal = c.local_normal_at(p);
            assert_eq!(normal, n);
        }
    }
}
