mod tests {
    use tracer::matrix::Matrix;
    use tracer::group::Group;

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
}