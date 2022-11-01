mod tests {
    use tracer::group::Group;
    use tracer::matrix::Matrix;
    use tracer::ray::Ray;
    use tracer::shape::{intersect, Shape, TestShape};
    use tracer::sphere::Sphere;
    use tracer::transforms::translation;
    use tracer::world::ShapeEnum;
    use tracer::{point, vector};

    #[test]
    fn test_create_group() {
        let g = Group::default();
        let identity = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);

        assert_eq!(g.transform, identity);
        assert_eq!(g.shapes.len(), 0);
    }

    #[test]
    fn test_add_child_to_group() {
        let mut g = Group::default();
        let s = ShapeEnum::Test(TestShape::new());
        g.add_child(&s);
        assert_eq!(g.shapes.len(), 1);
        assert_eq!(g.shapes[0], &s);
    }

    #[test]
    fn test_intersect_ray_group() {
        let g = Group::default();
        let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let xs = g.local_intersect(r);
        assert_eq!(xs.len(), 0)
    }

    #[test]
    fn test_intersect_non_empty() {
        let mut g = Group::default();
        let s1 = Sphere::default();
        let mut s2 = Sphere::default();
        s2.set_transform(translation(0., 0., -3.));
        let mut s3 = Sphere::default();
        s3.set_transform(translation(5., 0., 0.));
        let s1 = ShapeEnum::Sphere(s1);
        let s2 = ShapeEnum::Sphere(s2);
        let s3 = ShapeEnum::Sphere(s3);

        g.add_child(&s1);
        g.add_child(&s2);
        g.add_child(&s3);

        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let xs = intersect(&g, r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object, s2);
        assert_eq!(xs[1].object, s2);
        assert_eq!(xs[2].object, s1);
        assert_eq!(xs[3].object, s1);
    }
}
