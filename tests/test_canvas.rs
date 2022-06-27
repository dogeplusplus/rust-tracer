mod tests {
    use tracer::{Color};
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
    fn test_canvas_to_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = canvas_to_ppm(&c);
        let expected = vec![
            String::from("P3"),
            String::from("5 3"),
            String::from("255"),
        ];

        assert_eq!(ppm[0..3], expected)
    }

    #[test]
    fn test_canvas_to_ppm_pixels() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        write_pixel(&mut c, 0, 0, c1);
        write_pixel(&mut c, 2, 1, c2);
        write_pixel(&mut c, 4, 2, c3);

        let expected = vec![
            String::from("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"),
            String::from("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"),
            String::from("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"),
        ];
        
        let ppm = canvas_to_ppm(&c);
        assert_eq!(ppm[3..5], expected)
    }

    #[test]
    fn test_ppm_line_overflow() {
        let mut c = Canvas::new(10, 2);
        for i in 0..10 {
            for j in 0..2 {
                write_pixel(&mut c, i, j, Color::new(1.0, 0.8, 0.6))
            }
        }

        let expected = vec![
            String::from("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"),
            String::from("153 255 204 153 255 204 153 255 204 153 255 204 153"),
            String::from("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"),
            String::from("153 255 204 153 255 204 153 255 204 153 255 204 153"),
        ];

        let ppm = canvas_to_ppm(&c);
        assert_eq!(ppm, expected)
    }

}