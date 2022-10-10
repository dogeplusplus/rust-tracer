mod tests {
    use tracer::{
        intersections::prepare_computations,
        intersections::Intersection,
        lights::PointLight,
        materials::Material,
        point,
        ray::Ray,
        shape::Shape,
        sphere::Sphere,
        transforms::{scaling, translation},
        vector,
        world::{color_at, contains, intersect_world, is_shadowed, shade_hit, ShapeEnum, World},
        Color,
    };

    #[test]
    fn test_world() {
        let w = World::new();
        assert_eq!(w.objects.len(), 0);
        assert!(w.light.is_none());
    }

    #[test]
    fn test_default_world() {
        let light = PointLight::new(point(-10., 10., -10.), Color::new(1., 1., 1.));
        let mut s1 = Sphere::default();
        let mut m = Material::default();
        m.color = Color::new(0.8, 1.0, 0.6);
        m.diffuse = 0.7;
        m.specular = 0.2;
        s1.material = m;

        let mut s2 = Sphere::default();
        s2.transform = scaling(0.5, 0.5, 0.5);

        let w = World::default();
        assert_eq!(w.light.unwrap(), light);
        assert!(contains(&w, ShapeEnum::Sphere(s1)));
        assert!(contains(&w, ShapeEnum::Sphere(s2)));
    }

    #[test]
    fn test_intersect_world_ray() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let xs = intersect_world(&w, r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.);
    }

    #[test]
    fn test_shade_intersection() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let shape = w.objects[0];
        let i = Intersection::new(4., shape);
        let comps = prepare_computations(i, r);
        let c = shade_hit(&w, comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn test_shade_intersection_inside() {
        let mut w = World::default();
        w.light = Some(PointLight::new(point(0., 0.25, 0.), Color::new(1., 1., 1.)));
        let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let shape = w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = prepare_computations(i, r);
        let c = shade_hit(&w, comps);
        assert_eq!(c, Color::new(0.9049522, 0.9049522, 0.9049522));
    }

    #[test]
    fn test_intersect_world_ray_miss() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 1., 0.));
        let c = color_at(&w, r);
        assert_eq!(c, Color::new(0., 0., 0.));
    }

    #[test]
    fn test_intersect_world_ray_hit() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let c = color_at(&w, r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_intersection_behind_ray() {
        let mut w = World::default();

        match w.objects[0] {
            ShapeEnum::Sphere(ref mut sphere) => sphere.material.ambient = 1.,
            _ => panic!("Shape is not a sphere"),
        }
        match w.objects[1] {
            ShapeEnum::Sphere(ref mut sphere) => sphere.material.ambient = 1.,
            _ => panic!("Shape is not a sphere"),
        }
        let r = Ray::new(point(0., 0., 0.75), vector(0., 0., -1.));
        let c = color_at(&w, r);

        match w.objects[1] {
            ShapeEnum::Sphere(sphere) => {
                assert_eq!(c, sphere.material.color);
            }
            _ => panic!("Shape is not a sphere"),
        }
    }

    #[test]
    fn test_in_light_non_colinear() {
        let w = World::default();
        let p = point(0., 10., 0.);
        assert!(!is_shadowed(&w, p));
    }

    #[test]
    fn test_in_shadow_behind_object() {
        let w = World::default();
        let p = point(10., -10., 10.);
        assert!(is_shadowed(&w, p));
    }

    #[test]
    fn test_obj_behind_light() {
        let w = World::default();
        let p = point(-20., 20., -20.);
        assert!(!is_shadowed(&w, p));
    }

    #[test]
    fn test_point_between_light_and_obj() {
        let w = World::default();
        let p = point(-2., 2., -2.);
        assert!(!is_shadowed(&w, p));
    }

    #[test]
    fn test_shade_hit_sphere() {
        let mut w = World::new();
        w.light = Some(PointLight::new(point(0., 0., -10.), Color::new(1., 1., 1.)));
        let s1 = Sphere::default();
        let mut s2 = Sphere::default();
        s2.set_transform(translation(0., 0., 10.));
        w.objects = vec![ShapeEnum::Sphere(s1), ShapeEnum::Sphere(s2)];
        let r = Ray::new(point(0., 0., 5.), vector(0., 0., 1.));
        let i = Intersection::new(4., ShapeEnum::Sphere(s2));
        let comps = prepare_computations(i, r);
        let c = shade_hit(&w, comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_hit_offset_point() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let mut shape = Sphere::default();
        shape.set_transform(translation(0., 0., 1.));
        let i = Intersection::new(5., ShapeEnum::Sphere(shape));
        let comps = prepare_computations(i, r);
        assert!(comps.over_point.z < -f32::EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z)
    }
}
