mod tests {
    use tracer::shape::Shape;
    use tracer::cylinder::Cylinder;
    use std::iter::zip;
    use tracer::{point, normalize, ray::Ray};

    #[test]
    fn test_ray_miss_cylinder() {
        let cyl = Cylinder::default();

        let origins = vec![
            point(1., 0., 0.),
            point(0., 0., 0.),
            point(0., 0., -5.),
        ];
        
        let directions = vec![
            point(0., 1., 0.),
            point(0., 1., 0.),
            point(1., 1., 1.),
        ];

        for (origin, direction) in zip(origins, directions) {
            let direction = normalize(direction);
            let r = Ray::new(origin, direction);
            let xs = cyl.local_intersect(r);

            assert_eq!(xs.len(), 0);
        }
    }
}