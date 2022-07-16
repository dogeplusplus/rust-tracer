use crate::sphere::Sphere;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Intersection {
    pub t: f32,
    pub object: Sphere,
}

impl Intersection {
    pub fn new(t: f32, object: Sphere) -> Self {
        Intersection { t, object }
    }
}

pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    let valid_intersections: Vec<Intersection> = intersections
        .into_iter()
        .filter(|x| x.t >= 0.)
        .collect::<Vec<Intersection>>();
    if valid_intersections.len() > 0 {
        let mut min_intersection = valid_intersections[0];
        for intersect in valid_intersections {
            if intersect.t < min_intersection.t {
                min_intersection = intersect;
            }
        }
        return Some(min_intersection);
    }
    None
}
