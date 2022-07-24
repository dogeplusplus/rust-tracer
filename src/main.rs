use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use tracer::canvas::{canvas_to_ppm, write_pixel, Canvas};
use tracer::intersections::hit;
use tracer::lights::{lighting, PointLight};
use tracer::materials::Material;
use tracer::ray::{position, Ray};
use tracer::sphere::{intersect, normal_at, Sphere};
use tracer::transforms::{rotation_z, scaling, shearing};
use tracer::{normalize, point, Color};

fn main() {
    let ray_origin = point(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.;
    let canvas_pixels = 200;

    let mut c = Canvas::new(canvas_pixels, canvas_pixels);

    let pixel_size = wall_size / (canvas_pixels as f32);
    let half = wall_size / 2.;

    let mut shape = Sphere::default();
    let mut material = Material::default();
    material.ambient = 0.5;
    material.shininess = 20.;
    material.color = Color::new(0., 0.5, 0.5);
    shape.material = material;

    let transform =
        rotation_z(PI / 4.) * scaling(0.5, 1., 1.) * shearing(1., 0.5, -0.1, -1., -2., 0.1);
    shape.transform = transform;

    let light_position = point(0., 5., -1.);
    let light_color = Color::new(1., 1., 1.);
    let light = PointLight::new(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f32);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f32);
            let pos = point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, normalize(pos - ray_origin));

            let xs = intersect(shape, r);

            if let Some(h) = hit(xs) {
                let point = position(r, h.t);
                let normal = normal_at(h.object, point);
                let eye = -r.direction;
                let color = lighting(h.object.material, light, point, eye, normal);
                write_pixel(&mut c, x as usize, y as usize, color);
            }
        }
    }

    let ppm = canvas_to_ppm(&c);
    let mut f = File::create("output.svg").expect("Could not create file");
    for row in ppm {
        f.write_all(row.as_bytes()).expect("Could not write row.");
        f.write("\n".as_bytes()).expect("Could not write new line");
    }
}
