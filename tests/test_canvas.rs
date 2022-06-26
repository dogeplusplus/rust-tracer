mod tests {
    use std::fmt::write;

    use tracer::Color;
    use tracer::canvas::{Canvas, write_pixel, pixel_at, canvas_to_ppm};

    #[test]
    fn test_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.height, 10);
        assert_eq!(c.width, 20);

        let black = Color::new(0.0, 0.0, 0.0);

        for row in c.pixels {
            for pix in row {
                assert_eq!(pix, black);
            }
        }
    }

    #[test]
    fn test_write_pixel() {
        let mut c = Canvas::new(10, 20);
        let red = Color{red: 1.0, green: 0.0, blue: 0.0};

        write_pixel(&mut c, 2, 3, red);
        assert_eq!(pixel_at(&c, 2, 3), red)
    }

    #[test]
    fn canvas_to_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = canvas_to_ppm(c);
        let expected = "
        P3
        5 3
        255
        ";

        assert_eq!(ppm, expected)
    }

    #[test]
    fn canvas_to_ppm_pixels() {
        let c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        write_pixel(&mut c, 0, 0, c1);
        write_pixel(&mut c, 2, 1, c2);
        write_pixel(&mut c, 4, 2, c3);

        let expected = "
        255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
        ";
    }
}