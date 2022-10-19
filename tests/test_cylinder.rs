mod tests {
    use std::iter::zip;
    use tracer::cylinder::Cylinder;
    use tracer::shape::Shape;
    use tracer::vector;
    use tracer::{normalize, point, ray::Ray};

    #[test]
    fn test_ray_miss_cylinder() {
        let cyl = Cylinder::default();

        let origins = vec![point(1., 0., 0.), point(0., 0., 0.), point(0., 0., -5.)];

        let directions = vec![point(0., 1., 0.), point(0., 1., 0.), point(1., 1., 1.)];

        for (origin, direction) in zip(origins, directions) {
            let direction = normalize(direction);
            let r = Ray::new(origin, direction);
            let xs = cyl.local_intersect(r);

            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn test_ray_strike_cylinder() {
        let cyl = Cylinder::default();
        let origins = vec![
            point(1., 0., -5.),
            point(0., 0., -5.),
            point(0.5, 0., -5.),
        ];
        let directions = vec![
            vector(0., 0., 1.),
            vector(0., 0., 1.),
            vector(0.1, 1., 1.),
        ];
        let t0s = vec![5., 4., 6.808006];
        let t1s = vec![5., 6., 7.0886984];

        for ((origin, direction), (t0, t1)) in zip(zip(origins, directions), zip(t0s, t1s)) {
            let direction = normalize(direction);
            let r = Ray::new(origin, direction);
            let xs = cyl.local_intersect(r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, t0);
            assert_eq!(xs[1].t, t1);
        }
    }

    #[test]
    fn test_normal_cylinder() {
        let cyl = Cylinder::default();

        let points = vec![
            point(1., 0., 0.),
            point(0., 5., -1.),
            point(0., -2., 1.),
            point(-1., 1., 0.),
        ];
        let normals = vec![
            vector(1., 0., 0.),
            vector(0., 0., -1.),
            vector(0., 0., 1.),
            vector(-1., 0., 0.),
        ];

        for (point, normal) in zip(points, normals) {
            let n = cyl.local_normal_at(point);
            assert_eq!(n, normal);
        }
    }

    #[test]
    fn test_cylinder_min_max() {
        let cyl = Cylinder::default();
        assert_eq!(cyl.minimum, -f32::INFINITY);
        assert_eq!(cyl.maximum, f32::INFINITY);
    }

    #[test]
    fn test_intersect_constrained_cylinder() {
        let mut cyl = Cylinder::default();
        cyl.minimum = 1.;
        cyl.maximum = 2.;

        let origins = vec![
            point(0., 1.5, 0.),
            point(0., 3., -5.),
            point(0., 0., -5.),
            point(0., 2., -5.),
            point(0., 1., -5.),
            point(0., 1.5, -2.),
        ];

        let directions = vec![
            vector(0.1, 1., 0.),
            vector(0., 0., 1.),
            vector(0., 0., 1.),
            vector(0., 0., 1.),
            vector(0., 0., 1.),
            vector(0., 0., 1.),
        ];

        let counts = vec![0, 0, 0, 0, 0, 2];

        for ((origin, direction), count) in zip(zip(origins, directions), counts) {
            let r = Ray::new(origin, direction);
            let xs = cyl.local_intersect(r);
            assert_eq!(xs.len(), count);
        }
    }
}
