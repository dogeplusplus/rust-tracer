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
    let a = Tuple::new(4.3, -4.2, 3.1, 0.);

    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 0.);

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
    let p: Tuple = vector(4., -4., 3.);
    assert_eq!(p, Tuple::new(4., -4., 3., 0.));
}

#[test]
fn test_add_tuple() {
    let a1 = Tuple::new(3., -2., 5., 1.);
    let a2: Tuple = Tuple::new(-2., 3.0, 1., 0.);

    let sum = a1 + a2;
    assert_eq!(sum, Tuple::new(1.0, 1.0, 6.0, 1.0));
}

#[test]
fn test_sub_points() {
    let p1 = point(3., 2., 1.);
    let p2 = point(5., 6., 7.);

    let diff = p1 - p2;
    assert_eq!(diff, vector(-2., -4., -6.))
}

#[test]
fn sub_vec_from_point() {
    let p = point(3., 2., 1.);
    let v = vector(5., 6., 7.);

    let diff = p - v;
    assert_eq!(diff, point(-2., -4., -6.))
}

#[test]
fn sub_vec() {
    let v1 = vector(3., 2., 1.);
    let v2 = vector(5., 6., 7.);
    let diff = v1 - v2;

    assert_eq!(diff, vector(-2., -4., -6.))
}