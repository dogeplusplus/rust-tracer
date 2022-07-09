mod tests {
    use tracer::{point, vector};
    use tracer::transforms::translation;

    #[test]
    fn test_translation() {
        let p = point(-3., 4., 5.);
        let transform = translation(5., -3., 2.);

        let expected = point(2., 1., 7.);
        assert_eq!(transform * p, expected)
    }

    #[test]
    fn test_inverted_translation() -> Result<(), &'static str> {
        let transform = translation(5., -3., 2.);
        let inv = transform.inverse()?;
        let p = point(-3., 4., 5.);

        let expected = point(-8., 7., 3.);
        assert_eq!(inv * p, expected);
        Ok(())
    }

    #[test]
    fn test_transform_leaves_vectors() {
        let transform = translation(5., -3., 2.);
        let v = vector(-3., 4., 5.);

        assert_eq!(transform * v, v)
    }
}