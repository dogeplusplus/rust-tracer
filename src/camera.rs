use crate::{ray::Ray, normalize, point, matrix::Matrix, canvas::{Canvas, write_pixel}, world::{color_at, World}};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f32,
    pub pixel_size: f32,
    pub half_width: f32,
    pub half_height: f32,
    pub transform: Matrix<f32, 4, 4>,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Self {

        let half_view = f32::tan(field_of_view / 2.);
        let aspect = hsize as f32 / vsize as f32;

        let mut half_width = half_view;
        let mut half_height = half_view / aspect;
        if aspect < 1. {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.) / hsize as f32;
        let transform = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);

        Camera {
            hsize,
            vsize,
            field_of_view,
            pixel_size,
            half_width,
            half_height,
            transform,
        }
    }
}

pub fn ray_for_pixel(camera: Camera, px: usize, py: usize) -> Result<Ray, &'static str> {
    let xoffset = (px as f32 + 0.5) * camera.pixel_size;
    let yoffset = (py as f32 + 0.5) * camera.pixel_size;

    let world_x = camera.half_width - xoffset;
    let world_y = camera.half_height - yoffset;

    let pixel = camera.transform.inverse()? * point(world_x, world_y, -1.);
    let origin = camera.transform.inverse()? * point(0., 0., 0.);
    let direction = normalize(pixel - origin);

    Ok(Ray::new(origin, direction))
}

pub fn render(camera: Camera, world: World) -> Result<Canvas, &'static str> {
    let mut image = Canvas::new(camera.hsize, camera.vsize);

    for y in 0..camera.vsize {
        for x in 0..camera.hsize {
            let ray = ray_for_pixel(camera, x, y)?;
            let color = color_at(&world, ray);
            write_pixel(&mut image, x, y, color);
        }
    }

    Ok(image)
}