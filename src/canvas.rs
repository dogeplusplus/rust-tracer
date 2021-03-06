use crate::Color;

const MAX_PPM_LEN: usize = 70;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub height: usize,
    pub width: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::new();
        for _ in 0..height {
            pixels.push(vec![Color::new(0.0, 0.0, 0.0); width as usize]);
        }

        Canvas {
            width,
            height,
            pixels,
        }
    }
}

pub fn write_pixel(canvas: &mut Canvas, x: usize, y: usize, color: Color) {
    canvas.pixels[y][x] = color;
}

pub fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Color {
    canvas.pixels[y][x]
}

pub fn canvas_to_ppm(canvas: &Canvas) -> Vec<String> {
    let mut result = Vec::new();

    // Define header
    result.push(String::from("P3"));
    result.push(format!("{} {}", canvas.width, canvas.height));
    result.push(String::from("255"));

    for row in &canvas.pixels {
        let mut row_txt = String::new();
        for pix in row {
            let red_u8 = (pix.red.clamp(0.0, 1.0) * 255.) as u8;
            let green_u8 = (pix.green.clamp(0.0, 1.0) * 255.) as u8;
            let blue_u8 = (pix.blue.clamp(0.0, 1.0) * 255.) as u8;

            for color in [red_u8, green_u8, blue_u8].iter() {
                let color_str = format!("{}", color);
                if row_txt.len() + color_str.len() + 1 > MAX_PPM_LEN {
                    result.push(row_txt.to_string());
                    row_txt = String::new();
                } else if !row_txt.is_empty() {
                    row_txt.push(' ');
                }
                row_txt.push_str(&color_str);
            }
        }
        if !row_txt.is_empty() {
            result.push(row_txt);
        }
    }
    result
}
