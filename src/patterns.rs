use crate::{Color, Tuple, matrix::Matrix, world::ShapeEnum};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternType {
    Stripe(StripePattern),
    Test(),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pattern {
    pattern: PatternType,
    pub transform: Matrix<f32, 4, 4>,
}


impl Pattern {
    pub fn new(pattern: PatternType) -> Self {
        let transform = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);
        Pattern{pattern, transform}
    }

    pub fn set_transform(&mut self, transform: Matrix<f32, 4, 4>) {
        self.transform = transform;
    }

    pub fn pattern_at(&mut self, point: Tuple) -> Color {
        Color::new(point.x, point.y, point.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TestPattern {
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StripePattern {
    pub a: Color,
    pub b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn pattern_at(&self, point: Tuple) -> Color {
        if f32::floor(point.x) as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

pub struct GradientPattern {
    pub a: Color,
    pub b: Color,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn pattern_at(&self, point: Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x - f32::floor(point.x);
        self.a + distance * fraction
    }
}

pub fn pattern_at_shape(mut pattern: Pattern, shape: ShapeEnum, point: Tuple) -> Color {
    let pattern_inv = pattern.transform.inverse().unwrap();
    let shape_inv = match shape {
        ShapeEnum::Plane(plane) => plane.transform.inverse().unwrap(),
        ShapeEnum::Sphere(sphere) => sphere.transform.inverse().unwrap(),
    } ;
    let world_point = shape_inv * point;
    let pattern_point = pattern_inv * world_point;
    pattern.pattern_at(pattern_point)
}