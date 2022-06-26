use crate::Color;

#[derive(Debug,Clone)]
pub struct Canvas {
    pub height: u32,
    pub width: u32,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(height: u32, width: u32) -> Self {
        let mut pixels = Vec::new();
        for _ in 0..height {
            pixels.push(vec![Color::new(0.0, 0.0, 0.0); width as usize]);
        }

        Canvas { height, width, pixels }
    }
}

pub fn write_pixel(canvas: &mut Canvas, row: usize, col: usize, color: Color) {
    canvas.pixels[row][col] = color;
} 

pub fn pixel_at(canvas: &Canvas, row: usize, col: usize) -> Color {
    canvas.pixels[row][col]
}