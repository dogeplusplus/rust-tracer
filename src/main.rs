use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use tracer::camera::render;
use tracer::camera::Camera;
use tracer::canvas::canvas_to_ppm;
use tracer::lights::PointLight;
use tracer::materials::Material;
use tracer::patterns::GradientPattern;
use tracer::patterns::RadialGradient;
use tracer::patterns::StripePattern;
use tracer::patterns::{CheckerPattern, Pattern, PatternType, RingPattern};
use tracer::plane::Plane;
use tracer::sphere::Sphere;
use tracer::transforms::rotation_z;
use tracer::transforms::{rotation_x, scaling, shearing, translation, view_transform};
use tracer::world::{ShapeEnum, World};
use tracer::{point, vector, Color};

fn main() -> Result<(), &'static str> {

    // Middle Sphere
    let mut middle = Sphere::default();
    middle.transform = translation(-0.5, 1., 0.5);
    middle.material = Material::default();
    middle.material.color = Color::new(0.1, 1., 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle.material.reflective = 1.;

    let stripe = PatternType::Stripe(StripePattern::new(
        Color::new(1.0, 0.5, 0.0),
        Color::new(0., 0.5, 0.5),
    ));
    let mut stripe_pattern = Pattern::new(stripe);
    stripe_pattern.set_transform(scaling(0.5, 0.5, 0.5));
    middle.material.pattern = Some(stripe_pattern);
    middle.material.transparency = 0.9;
    middle.material.reflective = 0.9;

    // Interior Sphere
    let mut interior = Sphere::default();
    interior.transform = translation(-0.5, 1., 1.5) * scaling(0.5, 0.5, 0.5);
    interior.material = Material::default();
    interior.material.color = Color::new(0., 0.5, 0.8);
    interior.material.reflective = 0.9;
    interior.material.transparency = 0.2;
    
    let checker = PatternType::Checker(CheckerPattern::new(
        Color::new(0., 1.0, 0.5),
        Color::new(0.5, 0., 0.9)
    ));
    let mut checker_pattern = Pattern::new(checker);
    checker_pattern.set_transform(
        rotation_x(PI / 4.) 
        * rotation_z(PI / 4.)
        * scaling(0.2, 0.5, 2.0)
    );
    interior.material.pattern = Some(checker_pattern);

    // Left Sphere
    let mut left = Sphere::default();
    left.transform = translation(-1., 0., -1.75)
        * scaling(0.5, 0.5, 0.5)
        * shearing(0.2, 0.2, 0.2, 0.2, 0.2, 0.2);
    left.material = Material::default();
    left.material.color = Color::new(0., 0.5, 0.9);
    left.material.diffuse = 0.7;
    left.material.specular = 1.;

    let gradient = PatternType::Gradient(GradientPattern::new(
        Color::new(1.0, 0.2, 0.),
        Color::new(0., 0.2, 1.0),
    ));
    let gradient_pattern = Pattern::new(gradient);
    left.material.pattern = Some(gradient_pattern);

    let mut floor = Plane::default();
    floor.transform = translation(0., 0., -0.75);

    let checker = PatternType::Checker(CheckerPattern::new(
        Color::new(1., 1., 1.),
        Color::new(0.0, 0.0, 0.0),
    ));
    floor.material.pattern = Some(Pattern::new(checker));
    floor.material.reflective = 0.9;

    let mut world = World::default();
    world.objects = vec![
        ShapeEnum::Sphere(middle),
        ShapeEnum::Sphere(left),
        ShapeEnum::Plane(floor),
        ShapeEnum::Sphere(interior),
    ];
    world.light = Some(PointLight::new(
        point(-10., 10., -10.),
        Color::new(1., 1., 1.),
    ));
    let mut camera = Camera::new(1000, 500, PI / 3.);
    camera.transform = view_transform(point(0., 1.5, -5.), point(0., 1., 0.), vector(0., 1., 0.));
    let canvas = render(camera, world)?;
    let ppm = canvas_to_ppm(&canvas);
    let mut f = File::create("output.svg").expect("Could not create file");
    for row in ppm {
        f.write_all(row.as_bytes()).expect("Could not write row.");
        f.write("\n".as_bytes()).expect("Could not write new line");
    }
    Ok(())
}
