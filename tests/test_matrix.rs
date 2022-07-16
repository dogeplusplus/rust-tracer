mod tests {
    use tracer::matrix::Matrix;
    use tracer::Tuple;

    #[test]
    fn test_matrix_constructor() {
        let m = Matrix::new([
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(m[0][0], 1.);
        assert_eq!(m[0][3], 4.);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);
    }

    #[test]
    fn test_matrix_construct_2x2() {
        let m = Matrix::new([[-3., 5.], [1., -2.]]);

        assert_eq!(m[0][0], -3.);
        assert_eq!(m[0][1], 5.);
        assert_eq!(m[1][0], 1.);
        assert_eq!(m[1][1], -2.);
    }

    #[test]
    fn test_matrix_construct_3x3() {
        let m = Matrix::new([[-3., 5., 0.], [1., -2., -7.], [0., 1., 1.]]);

        assert_eq!(m[0][0], -3.);
        assert_eq!(m[1][1], -2.);
        assert_eq!(m[2][2], 1.);
    }

    #[test]
    fn test_matrix_equality() {
        let a = Matrix::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 10., 11., 12.],
            [13., 14., 15., 16.],
        ]);

        let b = Matrix::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 10., 11., 12.],
            [13., 14., 15., 16.],
        ]);

        assert_eq!(a, b)
    }

    #[test]
    fn test_matrix_inequality() {
        let a = Matrix::new([
            [1., 2., 3., 5.],
            [5., 6., 7., 8.],
            [9., 10., 11., 12.],
            [13., 14., 15., 16.],
        ]);

        let b = Matrix::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 10., 11., 12.],
            [13., 14., 15., 16.],
        ]);

        assert_ne!(a, b)
    }

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        let b = Matrix::new([
            [-2., 1., 2., 3.],
            [3., 2., 1., -1.],
            [4., 3., 6., 5.],
            [1., 2., 7., 8.],
        ]);

        let expected = Matrix::new([
            [20., 22., 50., 48.],
            [44., 54., 114., 108.],
            [40., 58., 110., 102.],
            [16., 26., 46., 42.],
        ]);

        let output = a * b;

        assert_eq!(output, expected)
    }

    #[test]
    fn test_multiplication_scalar() {
        let a = Matrix::new([
            [1., 2., 3., 4.],
            [2., 4., 4., 2.],
            [8., 6., 4., 1.],
            [0., 0., 0., 1.],
        ]);
        let b = 2.;
        let expected = Matrix::new([
            [2., 4., 6., 8.],
            [4., 8., 8., 4.],
            [16., 12., 8., 2.],
            [0., 0., 0., 2.],
        ]);
        let output = a * b;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_multiplication_tuple() {
        let a = Matrix::new([
            [1., 2., 3., 4.],
            [2., 4., 4., 2.],
            [8., 6., 4., 1.],
            [0., 0., 0., 1.],
        ]);

        let b = Tuple::new(1., 2., 3., 1.);
        let expected = Tuple::new(18., 24., 33., 1.);

        assert_eq!(expected, a * b);
    }

    #[test]
    fn test_multiplication_identity() {
        let a = Matrix::new([
            [0., 1., 2., 4.],
            [1., 2., 4., 8.],
            [2., 4., 8., 16.],
            [4., 8., 16., 32.],
        ]);

        let identity = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);
        let output = a.clone() * identity;
        assert_eq!(output, a);
    }

    #[test]
    fn test_transpose() {
        let a = Matrix::new([
            [0., 9., 3., 0.],
            [9., 8., 0., 8.],
            [1., 8., 5., 3.],
            [0., 0., 5., 8.],
        ]);
        let expected = Matrix::new([
            [0., 9., 1., 0.],
            [9., 8., 8., 0.],
            [3., 0., 5., 5.],
            [0., 8., 3., 8.],
        ]);

        let output = a.transpose();
        assert_eq!(expected, output)
    }

    #[test]
    fn test_transpose_identity() {
        let identity = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);
        let output = identity.transpose();
        assert_eq!(identity, output)
    }

    #[test]
    fn test_determinant_2x2() {
        let a = Matrix::new([[1., 5.], [-3., 2.]]);

        let det = a.determinant();
        assert_eq!(17., det)
    }

    #[test]
    fn test_submatrix() {
        let a = Matrix::new([[1., 5., 0.], [-3., 2., 7.], [0., 6., -3.]]);
        let output = a.submatrix(0, 2);
        let expected = Matrix::new([[-3., 2.], [0., 6.]]);
        assert_eq!(expected, output)
    }

    #[test]
    fn test_submatrix_4x4() {
        let a = Matrix::new([
            [-6., 1., 1., 6.],
            [-8., 5., 8., 6.],
            [-1., 0., 8., 2.],
            [-7., 1., -1., 1.],
        ]);
        let output = a.submatrix(2, 1);
        let expected = Matrix::new([[-6., 1., 6.], [-8., 8., 6.], [-7., -1., 1.]]);

        assert_eq!(expected, output)
    }

    #[test]
    fn test_minor() {
        let a = Matrix::new([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]);

        let b = a.submatrix(1, 0);
        let det_b = b.determinant();
        let minor_a = a.minor(1, 0);

        assert_eq!(det_b, minor_a)
    }

    #[test]
    fn test_cofactor() {
        let a = Matrix::new([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]);

        let min_1 = a.minor(0, 0);
        let cof_1 = a.cofactor(0, 0);
        let min_2 = a.minor(1, 0);
        let cof_2 = a.cofactor(1, 0);

        assert_eq!(min_1, -12.);
        assert_eq!(cof_1, -12.);
        assert_eq!(min_2, 25.);
        assert_eq!(cof_2, -25.);
    }

    #[test]
    fn test_determinant_3x3() {
        let a = Matrix::new([[1., 2., 6.], [-5., 8., -4.], [2., 6., 4.]]);

        assert_eq!(a.cofactor(0, 0), 56.);
        assert_eq!(a.cofactor(0, 1), 12.);
        assert_eq!(a.cofactor(0, 2), -46.);
        assert_eq!(a.determinant(), -196.);
    }

    #[test]
    fn test_determinant_4x4() {
        let a = Matrix::new([
            [-2., -8., 3., 5.],
            [-3., 1., 7., 3.],
            [1., 2., -9., 6.],
            [-6., 7., 7., -9.],
        ]);

        assert_eq!(a.cofactor(0, 0), 690.);
        assert_eq!(a.cofactor(0, 1), 447.);
        assert_eq!(a.cofactor(0, 2), 210.);
        assert_eq!(a.cofactor(0, 3), 51.);
        assert_eq!(a.determinant(), -4071.);
    }

    #[test]
    fn test_invertible() {
        let a = Matrix::new([
            [6., 4., 4., 4.],
            [5., 5., 7., 6.],
            [4., -9., 3., -7.],
            [9., 1., 7., -6.],
        ]);
        assert_eq!(a.determinant(), -2120.);
        assert!(a.invertible())
    }

    #[test]
    fn test_non_invertible() {
        let a = Matrix::new([
            [-4., 2., -2., -3.],
            [9., 6., 2., 6.],
            [0., -5., 1., -5.],
            [0., 0., 0., 0.],
        ]);

        assert_eq!(a.determinant(), 0.);
        assert!(!a.invertible())
    }

    #[test]
    fn test_inverse() -> Result<(), &'static str> {
        let a = Matrix::new([
            [-5., 2., 6., -8.],
            [1., -5., 1., 8.],
            [7., 7., -6., -7.],
            [1., -3., 7., 4.],
        ]);

        let b = a.inverse()?;

        assert_eq!(a.determinant(), 532.);
        assert_eq!(a.cofactor(2, 3), -160.);
        assert_eq!(b[3][2], -160. / 532.);
        assert_eq!(a.cofactor(3, 2), 105.);
        assert_eq!(b[2][3], 105. / 532.);

        let expected = Matrix::new([
            [116., 240., 128., -24.],
            [-430., -775., -236., 277.],
            [-42., -119., -28., 105.],
            [-278., -433., -160., 163.],
        ]) / 532.;

        assert_eq!(b, expected);
        Ok(())
    }

    #[test]
    fn test_inverse_2() -> Result<(), &'static str> {
        let a = Matrix::new([
            [9., 3., 0., 9.],
            [-5., -2., -6., -3.],
            [-4., 9., 6., 4.],
            [-7., 6., 6., 2.],
        ]);

        let b = a.inverse()?;
        let expected = Matrix::new([
            [-66., -126., 234., -360.],
            [-126., 54., 594., -540.],
            [-47., -237., -177., 210.],
            [288., 108., -432., 540.],
        ]) / 1620.;
        assert_eq!(b, expected);
        Ok(())
    }

    #[test]
    fn test_inverse_3() -> Result<(), &'static str> {
        let a = Matrix::new([
            [8., -5., 9., 2.],
            [7., 5., 6., 1.],
            [-6., 0., 9., 6.],
            [-3., 0., -9., -4.],
        ]);

        let b = a.inverse()?;
        let expected = Matrix::new([
            [90., 90., 165., 315.],
            [45., -72., -15., -18.],
            [-210., -210., -255., -540.],
            [405., 405., 450., 1125.],
        ]) / -585.;
        assert_eq!(b, expected);
        Ok(())
    }

    #[test]
    fn test_invert_multiplication() -> Result<(), &'static str> {
        let a = Matrix::new([
            [3., -9., 7., 3.],
            [3., -8., 2., -9.],
            [-4., 4., 4., 1.],
            [-6., 5., -1., 1.],
        ]);
        let b = Matrix::new([
            [8., 2., 2., 2.],
            [3., -1., 7., 0.],
            [7., 0., 5., 4.],
            [6., -2., 0., 5.],
        ]);

        let c = a * b;
        let diff = c * b.inverse()? - a;
        assert!(diff.norm() < 1e-5);
        Ok(())
    }

    #[test]
    fn test_norm() {
        let a = Matrix::new([
            [3., -9., 7., 3.],
            [3., -8., 2., -9.],
            [-4., 4., 4., 1.],
            [-6., 5., -1., 1.],
        ]);
        let norm = a.norm();
        let expected = 20.445048300260872;

        assert_eq!(norm, expected)
    }
}
