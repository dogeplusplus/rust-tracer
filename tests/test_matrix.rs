mod tests {
    use tracer::matrix::{Matrix};
    use tracer::Tuple;

    #[test]
    fn test_matrix_constructor() {
        let m = Matrix::new(vec![
            vec![1., 2., 3., 4.],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9., 10., 11., 12.],
            vec![13.5, 14.5, 15.5, 16.5],
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
        let m = Matrix::new(vec![
            vec![-3., 5.],
            vec![1., -2.],
        ]);

        assert_eq!(m[0][0], -3.);
        assert_eq!(m[0][1], 5.);
        assert_eq!(m[1][0], 1.);
        assert_eq!(m[1][1], -2.);
    }

    #[test]
    fn test_matrix_construct_3x3() {
        let m = Matrix::new(vec![
            vec![-3., 5., 0.],
            vec![1., -2., -7.],
            vec![0., 1., 1.],
        ]);

        assert_eq!(m[0][0], -3.);
        assert_eq!(m[1][1], -2.);
        assert_eq!(m[2][2], 1.);
    }

    #[test]
    fn test_matrix_equality() {
        let a = Matrix::new(vec![
            vec![1., 2., 3., 4.],
            vec![5., 6., 7., 8.],
            vec![9., 10., 11., 12.],
            vec![13., 14., 15., 16.],
        ]);

        let b = Matrix::new(vec![
            vec![1., 2., 3., 4.],
            vec![5., 6., 7., 8.],
            vec![9., 10., 11., 12.],
            vec![13., 14., 15., 16.],
        ]);

        assert_eq!(a, b)
    }

    #[test]
    fn test_matrix_inequality() {
        let a = Matrix::new(vec![
            vec![1., 2., 3., 5.],
            vec![5., 6., 7., 8.],
            vec![9., 10., 11., 12.],
            vec![13., 14., 15., 16.],
        ]);
        
        let b = Matrix::new(vec![
            vec![1., 2., 3., 4.],
            vec![5., 6., 7., 8.],
            vec![9., 10., 11., 12.],
            vec![13., 14., 15., 16.],
        ]);

        assert_ne!(a, b)
    }
    
    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix::new(vec![
            vec![1., 2., 3., 4.],
            vec![5., 6., 7., 8.],
            vec![9., 8., 7., 6.],
            vec![5., 4., 3., 2.],
        ]);

        let b = Matrix::new(vec![
            vec![-2., 1., 2., 3.],
            vec![3., 2., 1., -1.],
            vec![4., 3., 6., 5.],
            vec![1., 2., 7., 8.],
        ]);

        let expected = Matrix::new(vec![
            vec![20., 22., 50., 48.],
            vec![44., 54., 114., 108.],
            vec![40., 58., 110., 102.],
            vec![16., 26., 46., 42.],
        ]);

        let output = a * b;

        assert_eq!(output, expected)
    }

    #[test]
    fn test_multiplication_scalar() {
        let a = Matrix::new(vec![
            vec![1., 2., 3., 4.],
            vec![2., 4., 4., 2.],
            vec![8., 6., 4., 1.],
            vec![0., 0., 0., 1.],
        ]);
        let b = 2.;
        let expected = Matrix::new(vec![
            vec![2., 4., 6., 8.],
            vec![4., 8., 8., 4.],
            vec![16., 12., 8., 2.],
            vec![0., 0., 0., 2.],
        ]);
        let output = a * b;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_multiplication_tuple() {
        let a = Matrix::new(vec![
            vec![1., 2., 3., 4.],
            vec![2., 4., 4., 2.],
            vec![8., 6., 4., 1.],
            vec![0., 0., 0., 1.],
        ]);

        let b = Tuple::new(1., 2., 3., 1.);
        let expected = Tuple::new(18., 24., 33., 1.);

        assert_eq!(expected, a * b);
    }

}