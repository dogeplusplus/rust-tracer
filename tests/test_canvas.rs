mod tests {
    use tracer::Color;
    use tracer::canvas::{Canvas, write_pixel, pixel_at};

    fn test_canvas() {
        let c = Canvas{height: 10, width: 20};
        assert_eq!(c.height, 10);
        assert_eq!(c.width, 20);

        for pix in c.pixels {
            assert_eq!(pix, Color{red: 0.0, green: 0.0, blue: 0.0} )
        }
    }

    fn test_write_pixel() {
        let c = Canvas{height: 10, width: 20};
        let red = Color{red: 1.0, green: 0.0, blue: 0.0};

        write_pixel(c, 2, 3, red);
        assert_eq!(pixel_at(c, 2, 3), red)
    }
}