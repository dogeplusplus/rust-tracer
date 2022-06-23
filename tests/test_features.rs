use tracer::{Tuple, point, vector};

#[test]
fn test_vector_raw() {
    let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 1.0);

    assert!(a.is_vector());
    assert_eq!(a.is_point(), false);
}

#[test]
fn test_point_raw() {
    let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 0.0);

    assert!(a.is_point());
    assert_eq!(a.is_vector(), false);
}

#[test]
fn test_point() {
    let p: Tuple = point(4., -4., 3.);
    assert_eq!(p, Tuple::new(4., -4., 3., 1.));
}

#[test]
fn test_vector() {
    let p: Tuple = vector(4.0, -4.0, 3.0);
    assert_eq!(p, Tuple::new(4., -4., 3., 0.));
}