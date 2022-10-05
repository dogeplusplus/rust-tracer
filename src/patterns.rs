use crate::{Color, Tuple, world::ShapeEnum, matrix::Matrix};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix<f32, 4, 4>,
}

impl Pattern {
    pub fn set_transform(&mut self, transform: Matrix<f32, 4, 4>) {
        self.transform = transform;
    }
}

pub fn stripe_pattern(a: Color, b: Color) -> Pattern {
    let transform = Matrix::new([
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ]);
    Pattern { a, b, transform }
}

pub fn stripe_at(pattern: Pattern, point: Tuple) -> Color {
    if f32::floor(point.x) as i32 % 2 == 0 {
        pattern.a
    } else {
        pattern.b
    }
}

pub fn stripe_at_object(pattern: Pattern, shape: ShapeEnum, point: Tuple) -> Color {

    let shape_invert = match shape {
        ShapeEnum::Sphere(sphere) => sphere.transform.inverse().unwrap(),
        ShapeEnum::Plane(plane) => plane.transform.inverse().unwrap(),
    };

    let obj_point = shape_invert * point;
    let pattern_point = pattern.transform.inverse().unwrap() * obj_point;

    stripe_at(pattern, pattern_point)
}