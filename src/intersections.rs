use super::float_eq;
use rays::Ray;
use spheres::Sphere;
use std::cmp;
use tuples::Tuple;
use world::World;

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub t: f32,
    pub object: Sphere,
    pub point: Option<Tuple>,
    pub eyev: Option<Tuple>,
    pub normalv: Option<Tuple>,
    pub inside: Option<bool>,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        float_eq(self.t, other.t) && self.object == other.object
    }
}

impl Eq for Intersection {}

impl cmp::PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Intersection) -> Option<cmp::Ordering> {
        if self.t < other.t {
            Some(cmp::Ordering::Less)
        } else if self.t > other.t {
            Some(cmp::Ordering::Greater)
        } else {
            Some(cmp::Ordering::Equal)
        }
    }
}

impl cmp::Ord for Intersection {
    fn cmp(&self, other: &Intersection) -> cmp::Ordering {
        if self.t < other.t {
            cmp::Ordering::Less
        } else if self.t > other.t {
            cmp::Ordering::Greater
        } else {
            cmp::Ordering::Equal
        }
    }
}

impl Intersection {
    pub fn new(t: f32, object: Sphere) -> Self {
        Intersection {
            t,
            object,
            eyev: None,
            normalv: None,
            point: None,
            inside: None,
        }
    }

    pub fn prepare_hit(&mut self, ray: Ray) {
        let position = ray.position(self.t);
        self.point = Some(position);
        let eyev = -ray.direction;
        self.eyev = Some(eyev);
        let normalv = self.object.normal_at(position);
        if normalv.dot(eyev) < 0.0 {
            self.inside = Some(true);
            self.normalv = Some(-normalv)
        } else {
            self.inside = Some(false);
            self.normalv = Some(normalv);
        }
    }
}

#[test]
fn test_precomputing_the_state_of_an_intersection() {
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape = Sphere::new();
    let mut hit = Intersection::new(4.0, shape);
    hit.prepare_hit(ray);
    assert_eq!(hit.point, Some(Tuple::point(0.0, 0.0, -1.0)));
    assert_eq!(hit.eyev, Some(Tuple::vector(0.0, 0.0, -1.0)));
    assert_eq!(hit.normalv, Some(Tuple::vector(0.0, 0.0, -1.0)));
}

#[test]
fn test_an_intersection_occurs_on_the_outside() {
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape = Sphere::new();
    let mut hit = Intersection::new(4.0, shape);
    hit.prepare_hit(ray);
    assert_eq!(hit.inside, Some(false));
}

#[test]
fn test_an_intersection_occurs_on_the_inside() {
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape = Sphere::new();
    let mut hit = Intersection::new(1.0, shape);
    hit.prepare_hit(ray);
    assert_eq!(hit.point, Some(Tuple::point(0.0, 0.0, 1.0)));
    assert_eq!(hit.eyev, Some(Tuple::vector(0.0, 0.0, -1.0)));
    assert_eq!(hit.inside, Some(true));
    assert_eq!(hit.normalv, Some(Tuple::vector(0.0, 0.0, -1.0)));
}

pub fn find_hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    intersections
        .iter()
        .filter(|inter| inter.t > 0.0)
        .min()
        .cloned()
}

#[test]
fn test_an_intersection_encapsulates_t_and_object() {
    let s = Sphere::new();
    let i = Intersection::new(3.5, s.clone());
    assert_eq!(i.t, 3.5);
    assert_eq!(i.object, s);
}

#[test]
fn test_aggregating_intersections() {
    let s = Sphere::new();
    let xs = s.intersections(vec![1.0, 2.0]);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 1.0);
    assert_eq!(xs[1].t, 2.0);
}

#[test]
fn test_the_hit_when_all_intersections_have_positive_t() {
    let s = Sphere::new();
    let xs = s.intersections(vec![1.0, 2.0]);
    let h = find_hit(xs);
    assert!(h.is_some());
    assert_eq!(h.unwrap().t, 1.0);
}

#[test]
fn test_the_hit_when_some_intersections_have_negative_t() {
    let s = Sphere::new();
    let xs = s.intersections(vec![1.0, -1.0]);
    let h = find_hit(xs);
    assert!(h.is_some());
    assert_eq!(h.unwrap().t, 1.0);
}

#[test]
fn test_the_hit_when_all_intersections_have_negative_t() {
    let s = Sphere::new();
    let xs = s.intersections(vec![-2.0, -1.0]);
    let h = find_hit(xs);
    assert!(h.is_none());
}

#[test]
fn test_the_hit_is_always_the_lowest_non_negative_intersection() {
    let s = Sphere::new();
    let xs = s.intersections(vec![5.0, 7.0, -3.0, 2.0]);
    let h = find_hit(xs);
    assert!(h.is_some());
    assert_eq!(h.unwrap().t, 2.0);
}

pub fn intersect_world(world: World, ray: Ray) -> Vec<Intersection> {
    let mut intersections = world
        .objects
        .iter()
        .flat_map(|object| ray.intersect(*object))
        .collect::<Vec<Intersection>>();
    intersections.sort_unstable();
    intersections
}

#[test]
fn test_intersect_a_world_with_a_ray() {
    let world = World::default();
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let xs = intersect_world(world, ray);
    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 4.5);
    assert_eq!(xs[2].t, 5.5);
    assert_eq!(xs[3].t, 6.0);
}
