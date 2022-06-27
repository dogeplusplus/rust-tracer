use crate::Color;

const MAX_PPM_LEN: usize = 70;

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
    result.push(format!("{} {}", canvas.height, canvas.width).to_string());
    result.push(String::from("255"));

    for row in &canvas.pixels {
        let mut row_txt = String::new();
        for pix in row {

            let red_u8 = (pix.red.clamp(0.0, 1.0) * 255.) as u8;
            let green_u8 = (pix.green.clamp(0.0, 1.0) * 255.) as u8;
            let blue_u8 = (pix.blue.clamp(0.0, 1.0) * 255.) as u8;

            let rgb = format!("{} {} {}", red_u8, green_u8, blue_u8);
            if row_txt.len() + rgb.len() + 1 > MAX_PPM_LEN {
                result.push(row_txt.to_string());
                row_txt = String::new();
            } else {
                if row_txt.len() > 0 {
                    row_txt.push_str(" ");
                }
                row_txt.push_str(&rgb);
            }
        }
        if row_txt.len() > 0 {
            result.push(row_txt.to_string());
        }
    }
    result
}