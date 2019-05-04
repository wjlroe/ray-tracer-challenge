use super::{float_eq, EPSILON};
use lighting::lighting;
use rays::Ray;
use shapes::Shape;
use std::cmp;
use tuples::Tuple;
use world::World;

#[derive(Clone, Debug)]
pub struct Intersection {
    pub t: f32,
    pub object: Shape,
    pub point: Option<Tuple>,
    pub over_point: Option<Tuple>,
    pub eyev: Option<Tuple>,
    pub normalv: Option<Tuple>,
    pub inside: Option<bool>,
    pub reflectv: Option<Tuple>,
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
    pub fn new(t: f32, object: Shape) -> Self {
        Intersection {
            t,
            object,
            eyev: None,
            normalv: None,
            point: None,
            over_point: None,
            inside: None,
            reflectv: None,
        }
    }

    pub fn prepare_hit(&mut self, ray: &Ray) {
        let mut position = ray.position(self.t);
        let eyev = -ray.direction;
        let normalv = self.object.normal_at(position);
        position = position + normalv * 0.0001;
        self.eyev = Some(eyev);
        self.point = Some(position);
        if normalv.dot(eyev) < 0.0 {
            self.inside = Some(true);
            self.normalv = Some(-normalv)
        } else {
            self.inside = Some(false);
            self.normalv = Some(normalv);
        }
        self.over_point =
            Some(self.point.unwrap() + self.normalv.unwrap() * EPSILON);
        if let Some(normalv) = self.normalv {
            self.reflectv = Some(ray.direction.reflect(normalv));
        }
    }

    pub fn reflected_color(&self, world: &World, remaining: i32) -> Tuple {
        if remaining <= 0 || self.object.material.reflective == 0.0 {
            Tuple::color(0.0, 0.0, 0.0)
        } else {
            let reflect_ray =
                Ray::new(self.point.unwrap(), self.reflectv.unwrap());
            let color = world.color_at(&reflect_ray, remaining - 1);
            color * self.object.material.reflective
        }
    }

    pub fn shade_hit(&self, world: &World, remaining: i32) -> Tuple {
        let is_shadowed = world.is_shadowed(self.over_point.unwrap());
        let surface = lighting(
            self.object.material,
            self.object,
            world.light_source.unwrap(),
            self.over_point.unwrap(),
            self.eyev.unwrap(),
            self.normalv.unwrap(),
            is_shadowed,
        );
        let reflected = self.reflected_color(world, remaining);
        surface + reflected
    }
}

#[cfg(test)]
use lighting::PointLight;
#[cfg(test)]
use matrices::Matrix4;
#[cfg(test)]
use shapes::*;
#[cfg(test)]
use REFLECTION_RECURSION_LIMIT;

#[test]
fn test_precomputing_the_state_of_an_intersection() {
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape = Sphere::new();
    let mut hit = Intersection::new(4.0, shape);
    hit.prepare_hit(&ray);
    assert_eq!(hit.point, Some(Tuple::point(0.0, 0.0, -1.0001)));
    assert_eq!(hit.eyev, Some(Tuple::vector(0.0, 0.0, -1.0)));
    assert_eq!(hit.normalv, Some(Tuple::vector(0.0, 0.0, -1.0)));
}

#[test]
fn test_an_intersection_occurs_on_the_outside() {
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape = Sphere::new();
    let mut hit = Intersection::new(4.0, shape);
    hit.prepare_hit(&ray);
    assert_eq!(hit.inside, Some(false));
}

#[test]
fn test_an_intersection_occurs_on_the_inside() {
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape = Sphere::new();
    let mut hit = Intersection::new(1.0, shape);
    hit.prepare_hit(&ray);
    assert_eq!(hit.point, Some(Tuple::point(0.0, 0.0, 1.0001)));
    assert_eq!(hit.eyev, Some(Tuple::vector(0.0, 0.0, -1.0)));
    assert_eq!(hit.inside, Some(true));
    assert_eq!(hit.normalv, Some(Tuple::vector(0.0, 0.0, -1.0)));
}

#[test]
fn test_the_point_is_offset() {
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut shape = Sphere::new();
    shape.transform = Matrix4::translation(0.0, 0.0, 1.0);
    let mut hit = Intersection::new(5.0, shape);
    hit.prepare_hit(&ray);
    assert!(hit.over_point.unwrap().z < -EPSILON / 2.0);
    assert!(hit.point.unwrap().z > hit.over_point.unwrap().z);
}

#[test]
fn test_precomputing_the_reflection_vector() {
    let shape = Plane::new();
    let ray = Ray::new(
        Tuple::point(0.0, 1.0, -1.0),
        Tuple::vector(0.0, -(2f32.sqrt()) / 2.0, 2f32.sqrt() / 2.0),
    );
    let mut hit = Intersection::new(2f32.sqrt(), shape);
    hit.prepare_hit(&ray);
    assert_eq!(
        hit.reflectv,
        Some(Tuple::vector(0.0, 2f32.sqrt() / 2.0, 2f32.sqrt() / 2.0))
    );
}

#[test]
fn test_the_reflected_color_for_a_non_reflective_material() {
    let world = World::default();
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut shape = world.objects[1].clone();
    shape.material.ambient = 1.0;
    let mut hit = Intersection::new(1.0, shape);
    hit.prepare_hit(&ray);
    assert_eq!(
        hit.reflected_color(&world, REFLECTION_RECURSION_LIMIT),
        Tuple::color(0.0, 0.0, 0.0)
    );
}

#[test]
fn test_the_reflected_color_for_a_reflective_material() {
    let mut world = World::default();
    let mut shape = Plane::new();
    shape.material.reflective = 0.5;
    shape.transform = Matrix4::translation(0.0, -1.0, 0.0);
    world.add_shape(shape);
    let ray = Ray::new(
        Tuple::point(0.0, 0.0, -3.0),
        Tuple::vector(0.0, -(2f32.sqrt()) / 2.0, 2f32.sqrt() / 2.0),
    );
    let mut hit = Intersection::new(2f32.sqrt(), shape);
    hit.prepare_hit(&ray);
    assert_eq!(
        hit.reflected_color(&world, REFLECTION_RECURSION_LIMIT),
        Tuple::color(0.19034, 0.23793, 0.14276)
    );
}

#[test]
fn test_the_reflected_color_at_the_maximum_recursive_depth() {
    let mut world = World::default();
    let mut shape = Plane::new();
    shape.material.reflective = 0.5;
    shape.transform = Matrix4::translation(0.0, -1.0, 0.0);
    world.add_shape(shape);
    let ray = Ray::new(
        Tuple::point(0.0, 0.0, -3.0),
        Tuple::vector(0.0, -2f32.sqrt() / 2.0, 2f32.sqrt() / 2.0),
    );
    let mut hit = Intersection::new(2f32.sqrt(), shape);
    hit.prepare_hit(&ray);
    assert_eq!(hit.reflected_color(&world, 0), Tuple::color(0.0, 0.0, 0.0));
}

#[test]
fn test_shade_hit_with_a_reflective_material() {
    let mut world = World::default();
    let mut shape = Plane::new();
    shape.material.reflective = 0.5;
    shape.transform = Matrix4::translation(0.0, -1.0, 0.0);
    world.add_shape(shape);
    let ray = Ray::new(
        Tuple::point(0.0, 0.0, -3.0),
        Tuple::vector(0.0, -(2f32.sqrt()) / 2.0, 2f32.sqrt() / 2.0),
    );
    let mut hit = Intersection::new(2f32.sqrt(), shape);
    hit.prepare_hit(&ray);
    let color = hit.shade_hit(&world, REFLECTION_RECURSION_LIMIT);
    assert_eq!(color, Tuple::color(0.87677, 0.92436, 0.82918));
}

#[test]
fn test_shading_an_intersection() {
    let world = World::default();
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape = world.objects[0].clone();
    let mut hit = Intersection::new(4.0, shape);
    hit.prepare_hit(&ray);
    let c = hit.shade_hit(&world, REFLECTION_RECURSION_LIMIT);
    assert_eq!(c, Tuple::color(0.38066, 0.47583, 0.2855));
}

#[test]
fn test_shading_an_intersection_from_the_inside() {
    let mut world = World::default();
    world.light_source = Some(PointLight::new(
        Tuple::point(0.0, 0.25, 0.0),
        Tuple::color(1.0, 1.0, 1.0),
    ));
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape = world.objects[1].clone();
    let mut hit = Intersection::new(0.5, shape);
    hit.prepare_hit(&ray);
    let c = hit.shade_hit(&world, REFLECTION_RECURSION_LIMIT);
    assert_eq!(c, Tuple::color(0.90502, 0.90502, 0.90502));
}

#[test]
fn test_when_shade_hit_is_given_an_intersection_in_shadow() {
    let mut world = World::new();
    world.light_source = Some(PointLight::new(
        Tuple::point(0.0, 0.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    ));
    let s1 = Sphere::new();
    world.objects.push(s1);
    let mut s2 = Sphere::new();
    s2.transform = Matrix4::translation(0.0, 0.0, 10.0);
    world.objects.push(s2);
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut hit = Intersection::new(4.0, s2);
    hit.prepare_hit(&ray);
    let c = hit.shade_hit(&world, REFLECTION_RECURSION_LIMIT);
    assert_eq!(c, Tuple::color(0.1, 0.1, 0.1));
}

pub fn find_hit<'a>(intersections: &[Intersection]) -> Option<Intersection> {
    intersections
        .iter()
        .filter(|inter| inter.t > 0.01)
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
    let h = find_hit(&xs);
    assert!(h.is_some());
    assert_eq!(h.unwrap().t, 1.0);
}

#[test]
fn test_the_hit_when_some_intersections_have_negative_t() {
    let s = Sphere::new();
    let xs = s.intersections(vec![1.0, -1.0]);
    let h = find_hit(&xs);
    assert!(h.is_some());
    assert_eq!(h.unwrap().t, 1.0);
}

#[test]
fn test_the_hit_when_all_intersections_have_negative_t() {
    let s = Sphere::new();
    let xs = s.intersections(vec![-2.0, -1.0]);
    let h = find_hit(&xs);
    assert!(h.is_none());
}

#[test]
fn test_the_hit_is_always_the_lowest_non_negative_intersection() {
    let s = Sphere::new();
    let xs = s.intersections(vec![5.0, 7.0, -3.0, 2.0]);
    let h = find_hit(&xs);
    assert!(h.is_some());
    assert_eq!(h.unwrap().t, 2.0);
}
