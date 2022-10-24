use crate::{magnitude, matrix::Matrix, shape::Shape, world::ShapeEnum, Color, Tuple};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternType {
    Ring(RingPattern),
    Gradient(GradientPattern),
    Stripe(StripePattern),
    Checker(CheckerPattern),
    Radial(RadialGradient),
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
        Pattern { pattern, transform }
    }

    pub fn set_transform(&mut self, transform: Matrix<f32, 4, 4>) {
        self.transform = transform;
    }

    pub fn pattern_at(&mut self, point: Tuple) -> Color {
        match self.pattern {
            PatternType::Gradient(grad) => grad.local_pattern_at(point),
            PatternType::Stripe(stripe) => stripe.local_pattern_at(point),
            PatternType::Checker(checker) => checker.local_pattern_at(point),
            PatternType::Ring(ring) => ring.local_pattern_at(point),
            PatternType::Radial(radial) => radial.local_pattern_at(point),
            PatternType::Test() => Color::new(point.x, point.y, point.z),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TestPattern {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StripePattern {
    pub a: Color,
    pub b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn local_pattern_at(&self, point: Tuple) -> Color {
        if f32::floor(point.x) as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientPattern {
    pub a: Color,
    pub b: Color,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn local_pattern_at(&self, point: Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x.abs() - point.x.abs().floor();
        self.a + distance * fraction
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RingPattern {
    pub a: Color,
    pub b: Color,
}

impl RingPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn local_pattern_at(&self, point: Tuple) -> Color {
        if (f32::floor(f32::sqrt(point.x * point.x + point.z * point.z)) as i32) % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CheckerPattern {
    pub a: Color,
    pub b: Color,
}

impl CheckerPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn local_pattern_at(&self, point: Tuple) -> Color {
        if (f32::floor(point.x) + f32::floor(point.y) + f32::floor(point.z)) as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

pub fn pattern_at_shape(mut pattern: Pattern, shape: ShapeEnum, point: Tuple) -> Color {
    let pattern_inv = pattern.transform.inverse().unwrap();
    let shape_inv = match shape {
        ShapeEnum::Plane(plane) => plane.get_transform().inverse().unwrap(),
        ShapeEnum::Sphere(sphere) => sphere.get_transform().inverse().unwrap(),
        ShapeEnum::Cube(cube) => cube.get_transform().inverse().unwrap(),
        ShapeEnum::Cylinder(cylinder) => cylinder.get_transform().inverse().unwrap(),
        ShapeEnum::Cone(cone) => cone.get_transform().inverse().unwrap(),
        ShapeEnum::Group(group) => group.get_transform().inverse().unwrap(),
    };
    let world_point = shape_inv * point;
    let pattern_point = pattern_inv * world_point;
    pattern.pattern_at(pattern_point)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RadialGradient {
    pub a: Color,
    pub b: Color,
}

impl RadialGradient {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn local_pattern_at(&self, point: Tuple) -> Color {
        let distance = self.b - self.a;
        let new = Tuple::new(point.x, point.y, point.z, 0.);
        let m = magnitude(new);
        let fraction = m - m.floor();
        self.a + distance * fraction
    }
}
