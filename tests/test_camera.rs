mod tests {
    use std::f32::consts::PI;
    use tracer::camera::{ray_for_pixel, render, Camera};
    use tracer::canvas::pixel_at;
    use tracer::matrix::Matrix;
    use tracer::transforms::{rotation_y, translation, view_transform};
    use tracer::world::World;
    use tracer::Color;
    use tracer::{magnitude, point, vector};

    #[test]
    fn test_camera_construction() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.;
        let c = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        let identity = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);

        assert_eq!(c.transform, identity);
    }

    #[test]
    fn test_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn test_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn test_ray_center_of_canvas() -> Result<(), String> {
        let c = Camera::new(201, 101, PI / 2.);
        let r = ray_for_pixel(c, 100, 50)?;
        assert_eq!(r.origin, point(0., 0., 0.));
        assert!(magnitude(r.direction - vector(0., 0., -1.)) < 1e-5);
        Ok(())
    }

    #[test]
    fn test_ray_corner_canvas() -> Result<(), String> {
        let c = Camera::new(201, 101, PI / 2.);
        let r = ray_for_pixel(c, 0, 0)?;
        assert_eq!(r.origin, point(0., 0., 0.));
        assert_eq!(r.direction, vector(0.6651864, 0.33259323, -0.66851234));
        Ok(())
    }

    #[test]
    fn test_ray_camera_transform() -> Result<(), String> {
        let mut c = Camera::new(201, 101, PI / 2.);
        c.transform = rotation_y(PI / 4.) * translation(0., -2., 5.);
        let r = ray_for_pixel(c, 100, 50)?;
        assert!(magnitude(r.origin - point(0., 2., -5.)) < 1e-5);
        assert!(
            magnitude(r.direction - vector(1. / f32::sqrt(2.), 0., -1. / f32::sqrt(2.))) < 1e-5
        );
        Ok(())
    }

    #[test]
    fn test_render_world() -> Result<(), String> {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.);
        let from = point(0., 0., -5.);
        let to = point(0., 0., 0.);
        let up = vector(0., 1., 0.);
        c.transform = view_transform(from, to, up);
        let image = render(c, w)?;
        assert_eq!(pixel_at(&image, 5, 5), Color::new(0.38066, 0.47583, 0.2855));
        Ok(())
    }
}
