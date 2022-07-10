mod tests {
    use std::f32::consts::PI;
    use tracer::{point, vector, magnitude};
    use tracer::transforms::{translation, scaling, rotation_x, rotation_y, rotation_z, shearing};

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

    #[test]
    fn test_scaling_point() {
        let transform = scaling(2., 3., 4.);
        let p = point(-4., 6., 8.);
        let expected = point(-8., 18., 32.);
        assert_eq!(transform * p, expected)
    }

    #[test]
    fn test_scaling_vector() {
        let transform = scaling(2., 3., 4.);
        let v = vector(-4., 6., 8.);
        let expected = vector(-8., 18., 32.);
        assert_eq!(transform * v, expected)
    }

    #[test]
    fn test_scaling_inverse() -> Result<(), &'static str> {
        let transform = scaling(2., 3., 4.);
        let inv = transform.inverse()?;
        let v = vector(-4., 6., 8.);
        let expected = vector(-2., 2., 2.);
        assert_eq!(inv * v, expected);
        Ok(())
    }

    #[test]
    fn test_reflection() {
        let transform = scaling(-1., 1., 1.);
        let p = point(2., 3., 4.);
        let expected = point(-2., 3., 4.);
        assert_eq!(transform * p, expected)
    }

    #[test]
    fn test_rotation_x() {
        let p = point(0., 1., 0.);
        let half_quarter = rotation_x(PI / 4.);
        let full_quarter = rotation_x(PI / 2.);
        let expected_half = point(0., f32::sqrt(2.) / 2., f32::sqrt(2.) / 2.);
        let expected_full = point(0., 0., 1.);
        let diff_half = half_quarter * p - expected_half;
        let diff_full = full_quarter * p - expected_full;
        assert!(magnitude(diff_half) < 1e-5);
        assert!(magnitude(diff_full) < 1e-5);
    }

    #[test]
    fn test_inverse_rotation() -> Result<(), &'static str> {
        let p = point(0., 1., 0.);
        let half_quarter = rotation_x(PI / 4.);
        let inv = half_quarter.inverse()?;
        let expected = point(0., f32::sqrt(2.) / 2., -f32::sqrt(2.) / 2.);
        let diff = inv * p - expected; 
        assert!(magnitude(diff) < 1e-5);
        Ok(())
    }

    #[test]
    fn test_rotation_y() {
        let p = point(0., 0., 1.);
        let half_quarter = rotation_y(PI / 4.);
        let full_quarter = rotation_y(PI / 2.);
        let expected_half = point(f32::sqrt(2.) / 2., 0., f32::sqrt(2.) / 2.);
        let expected_full = point(1., 0., 0.);
        let diff_half = half_quarter * p - expected_half;
        let diff_full = full_quarter * p - expected_full;
        assert!(magnitude(diff_half) < 1e-5);
        assert!(magnitude(diff_full) < 1e-5);
    }

    #[test]
    fn test_rotation_z() {
        let p = point(0., 1., 0.);
        let half_quarter = rotation_z(PI / 4.);
        let full_quarter = rotation_z(PI / 2.);
        let expected_half = point(-f32::sqrt(2.) / 2., f32::sqrt(2.) / 2., 0.);
        let expected_full = point(-1., 0., 0.);
        let diff_half = half_quarter * p - expected_half;
        let diff_full = full_quarter * p - expected_full;
        assert!(magnitude(diff_half) < 1e-5);
        assert!(magnitude(diff_full) < 1e-5);
    }

    #[test]
    fn test_shearing() {
        let transform = shearing(1., 0., 0., 0., 0., 0.);
        let p = point(2., 3., 4.);
        let expected = point(5., 3., 4.);
        assert_eq!(transform * p, expected)
    }

    #[test]
    fn test_shearing_xz() {
        let transform = shearing(0., 1., 0., 0., 0., 0.);
        let p = point(2., 3., 4.);
        let expected = point(6., 3., 4.);
        assert_eq!(transform * p, expected)
    }

    #[test]
    fn test_shearing_yx() {
        let transform = shearing(0., 0., 1., 0., 0., 0.);
        let p = point(2., 3., 4.);
        let expected = point(2., 5., 4.);
        assert_eq!(transform * p, expected)
    }

    #[test]
    fn test_shearing_yz() {
        let transform = shearing(0., 0., 0., 1., 0., 0.);
        let p = point(2., 3., 4.);
        let expected = point(2., 7., 4.);
        assert_eq!(transform * p, expected)
    }

    #[test]
    fn test_shearing_zx() {
        let transform = shearing(0., 0., 0., 0., 1., 0.);
        let p = point(2., 3., 4.);
        let expected = point(2., 3., 6.);
        assert_eq!(transform * p, expected)
    }

    #[test]
    fn test_shearing_zy() {
        let transform = shearing(0., 0., 0., 0., 0., 1.);
        let p = point(2., 3., 4.);
        let expected = point(2., 3., 7.);
        assert_eq!(transform * p, expected)
    }

    #[test]
    fn test_composition() {
        let p = point(1., 0., 1.);
        let a = rotation_x(PI / 2.);
        let b = scaling(5., 5., 5.);
        let c = translation(10., 5., 7.);

        let p2 = a * p;
        
        let diff = p2 - point(1., -1., 0.);
        assert!(magnitude(diff) < 1e-5);

        let p3 = b * p2;
        let diff = p3 - point(5., -5., 0.);
        assert!(magnitude(diff) < 1e-5);

        let p4 = c * p3;
        let diff = p4 - point(15., 0., 7.);
        assert!(magnitude(diff) < 1e-5);
    }

    #[test]
    fn test_composition_reverse() {
        let p = point(1., 0., 1.);
        let a = rotation_x(PI / 2.);
        let b = scaling(5., 5., 5.);
        let c = translation(10., 5., 7.);
        let t = c * b * a;
        assert_eq!(t * p, point(15., 0., 7.))
    }
}