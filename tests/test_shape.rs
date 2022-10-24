mod tests {
    use std::f32::consts::PI;

    use tracer::group::Group;
    use tracer::materials::Material;
    use tracer::matrix::Matrix;
    use tracer::ray::Ray;
    use tracer::shape::{intersect, normal_at, Shape};
    use tracer::transforms::{rotation_z, scaling, translation};
    use tracer::Tuple;
    use tracer::{point, vector};

    #[derive(Clone)]
    struct TestShape {
        material: Material,
        saved_ray: Ray,
        transform: Matrix<f32, 4, 4>,
        parent: Option<Group>,
    }
    impl TestShape {
        fn new() -> Self {
            let identity = Matrix::new([
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ]);
            TestShape {
                material: Material::default(),
                saved_ray: Ray {
                    origin: point(0., 0., 0.),
                    direction: vector(0., 0., 0.),
                },
                transform: identity,
                parent: None,
            }
        }
    }
    impl Shape for TestShape {
        fn get_transform(&self) -> Matrix<f32, 4, 4> {
            self.transform
        }

        fn set_transform(&mut self, transform: Matrix<f32, 4, 4>) {
            self.transform = transform
        }

        fn local_intersect(&self, _: Ray) -> Vec<tracer::intersections::Intersection> {
            Vec::new()
        }

        fn local_normal_at(&self, point: tracer::Tuple) -> Tuple {
            point
        }
    }

    #[test]
    fn test_default_transformation() {
        let identity = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);
        let s = TestShape::new();
        assert_eq!(s.get_transform(), identity);
    }

    #[test]
    fn test_set_transformation() {
        let mut s = TestShape::new();
        s.set_transform(translation(2., 3., 4.));
        assert_eq!(s.get_transform(), translation(2., 3., 4.));
    }

    #[test]
    fn test_default_material() {
        let s = TestShape::new();
        let m = s.material;
        assert_eq!(m, Material::default());
    }

    #[test]
    fn test_assign_material() {
        let mut s = TestShape::new();
        let mut m = Material::default();
        m.ambient = 1.;
        s.material = m;
        assert_eq!(s.material, m);
    }

    #[test]
    #[ignore]
    fn test_intersect_shape_with_ray() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = TestShape::new();
        s.set_transform(scaling(2., 2., 2.));
        let _xs = intersect(&s, r);
        assert_eq!(s.saved_ray.origin, point(0., 0., -2.5));
        assert_eq!(s.saved_ray.direction, vector(0., 0., 0.5));
    }

    #[test]
    #[ignore]
    fn test_intersect_translated_shape_with_ray() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = TestShape::new();
        s.set_transform(translation(5., 0., 0.));
        let _xs = intersect(&s, r);
        assert_eq!(s.saved_ray.origin, point(-5., 0., -5.));
        assert_eq!(s.saved_ray.direction, vector(0., 0., 1.));
    }

    #[test]
    fn test_normal_translated_shape() {
        let mut s = TestShape::new();
        s.set_transform(translation(0., 1., 0.));
        let n = normal_at(&s, point(0., 1.70711, -0.70711));
        assert_eq!(n, vector(0., 0.7071068, -0.70710677));
    }

    #[test]
    fn test_normal_transformed_shape() {
        let mut s = TestShape::new();
        let m = scaling(1., 0.5, 1.) * rotation_z(PI / 5.);
        s.set_transform(m);
        let n = normal_at(&s, point(0., f32::sqrt(2.) / 2., -f32::sqrt(2.) / 2.));
        assert_eq!(n, vector(-0.000000020444226, 0.97014254, -0.24253564));
    }

    #[test]
    fn test_shape_parent() {
        let s = TestShape::new();
        assert!(s.parent.is_none());
    }
}
