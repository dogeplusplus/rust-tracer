mod tests {
    use std::iter::zip;
    use tracer::cone::Cone;
    use tracer::ray::Ray;
    use tracer::shape::Shape;
    use tracer::{point, vector, normalize};

    #[test]
    fn test_intersect_cone_ray() {
        let shape = Cone::default();
        
        // Using value close to 5 due to floating point errors
        let origins = vec![
            point(0., 0., -5.),
            point(0., 0., -4.999),
            point(1., 1., -4.999),
        ];

        let directions = vec![
            vector(0., 0., 1.),
            vector(1., 1., 1.),
            vector(-0.5, -1., 1.),
        ];

        let t0s = vec![5., 8.658523, 4.5492673];
        let t1s = vec![5., 8.658523, 49.43874];

        for ((origin, direction), (t0, t1)) in zip(zip(origins, directions), zip(t0s, t1s)) {
            let direction = normalize(direction);
            let r = Ray::new(origin, direction);
            let xs = shape.local_intersect(r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, t0);
            assert_eq!(xs[1].t, t1);
        }
    }

    #[test]
    fn test_cone_parallel_ray() {
        let shape = Cone::default();
        let direction = normalize(vector(0., 1., 1.));
        let r = Ray::new(point(0., 0., -1.), direction);
        let xs = shape.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 0.35355338);
    }

    #[test]
    fn test_intersect_cone_cap() {
        let mut shape = Cone::default();
        shape.minimum = -0.5;
        shape.maximum = 0.5;
        shape.closed = true;
        
        let origins = vec![
            point(0., 0., -5.),
            point(0., 0., -0.25),
            point(0., 0., -0.25),
        ];

        let directions = vec![
            vector(0., 1., 0.),
            vector(0., 1., 1.),
            vector(0., 1., 0.),
        ];
        let counts = vec![0, 2, 4];

        for idx in 0..origins.len() {
            let direction = normalize(directions[idx]);
            let r = Ray::new(origins[idx], direction);
            let xs = shape.local_intersect(r);
            assert_eq!(xs.len(), counts[idx]);
        }
    }

    #[test]
    fn test_cone_normal() {
        let shape = Cone::default();

        let points = vec![
            point(0., 0., 0.),
            point(1., 1., 1.),
            point(-1., -1., 0.),
        ];
        let normals = vec![
            vector(0., 0., 0.),
            vector(1., -f32::sqrt(2.), 1.),
            vector(-1., 1., 0.),
        ];
        
        for idx in 0..points.len() {

            let n = shape.local_normal_at(points[idx]);
            assert_eq!(n, normals[idx]);
        }
    }

}