use crate::{Color, Tuple};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pattern {
    pub a: Color,
    pub b: Color,
}

pub fn stripe_pattern(a: Color, b: Color) -> Pattern {
    Pattern { a, b }
}

pub fn stripe_at(pattern: Pattern, point: Tuple) -> Color {
    if f32::floor(point.x) as i32 % 2 == 0 {
        pattern.a
    } else {
        pattern.b
    }
}