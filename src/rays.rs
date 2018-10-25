use super::EPSILON;
use matrices::{Matrix4, IDENTITY_MATRIX4};
use std::cmp;
use tuples::Tuple;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Sphere {
    pub transform: Matrix4,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transform: IDENTITY_MATRIX4,
        }
    }

    pub fn intersections(&self, ts: Vec<f32>) -> Vec<Intersection> {
        ts.iter()
            .map(|t| Intersection::new(*t, self.clone()))
            .collect::<Vec<_>>()
    }
}

#[test]
fn test_a_spheres_default_transformation() {
    let s = Sphere::new();
    assert_eq!(s.transform, IDENTITY_MATRIX4);
}

#[test]
fn test_changing_a_spheres_transformation() {
    let mut s = Sphere::new();
    let t = Matrix4::translation(2.0, 3.0, 4.0);
    s.transform = t;
    assert_eq!(s.transform, t);
}

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f32) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn intersect(&self, sphere: Sphere) -> Vec<Intersection> {
        let ray = self.transform(sphere.transform.inverse());
        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            let i1 = Intersection::new(t1, sphere.clone());
            let i2 = Intersection::new(t2, sphere.clone());
            if t1 > t2 {
                vec![i2, i1]
            } else {
                vec![i1, i2]
            }
        }
    }

    pub fn transform(&self, m: Matrix4) -> Ray {
        Ray::new(m * self.origin, m * self.direction)
    }
}

#[test]
fn test_creating_and_querying_a_ray() {
    let origin = Tuple::point(1.0, 2.0, 3.0);
    let direction = Tuple::vector(4.0, 5.0, 6.0);
    let r = Ray::new(origin, direction);
    assert_eq!(r.origin, origin);
    assert_eq!(r.direction, direction);
}

#[test]
fn test_computing_a_point_from_a_distance() {
    let r = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));
    assert_eq!(r.position(0.0), Tuple::point(2.0, 3.0, 4.0));
    assert_eq!(r.position(1.0), Tuple::point(3.0, 3.0, 4.0));
    assert_eq!(r.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
    assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
}

#[test]
fn test_a_ray_intersects_a_sphere_at_two_points() {
    let r =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = r.intersect(s);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 6.0);
}

#[test]
fn test_a_ray_intersects_a_sphere_at_a_tangent() {
    let r =
        Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = r.intersect(s);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 5.0);
    assert_eq!(xs[1].t, 5.0);
}

#[test]
fn test_a_ray_misses_a_sphere() {
    let r =
        Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = r.intersect(s);
    assert_eq!(xs.len(), 0);
}

#[test]
fn test_a_ray_originates_inside_a_sphere() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = r.intersect(s);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -1.0);
    assert_eq!(xs[1].t, 1.0);
}

#[test]
fn test_a_sphere_is_behind_a_ray() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = r.intersect(s);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -6.0);
    assert_eq!(xs[1].t, -4.0);
}

#[test]
fn test_intersect_sets_the_object_on_the_intersection() {
    let r =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = r.intersect(s);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].object, s.clone());
    assert_eq!(xs[1].object, s.clone());
}

#[test]
fn test_translating_a_ray() {
    let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
    let m = Matrix4::translation(3.0, 4.0, 5.0);
    let r2 = r.transform(m);
    assert_eq!(r2.origin, Tuple::point(4.0, 6.0, 8.0));
    assert_eq!(r2.direction, Tuple::vector(0.0, 1.0, 0.0));
}

#[test]
fn test_scaling_a_ray() {
    let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
    let m = Matrix4::scaling(2.0, 3.0, 4.0);
    let r2 = r.transform(m);
    assert_eq!(r2.origin, Tuple::point(2.0, 6.0, 12.0));
    assert_eq!(r2.direction, Tuple::vector(0.0, 3.0, 0.0));
}

#[test]
fn test_intersecting_a_scaled_sphere_with_a_ray() {
    let r =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut s = Sphere::new();
    s.transform = Matrix4::scaling(2.0, 2.0, 2.0);
    let xs = r.intersect(s);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 3.0);
    assert_eq!(xs[1].t, 7.0);
}

#[test]
fn test_intersecting_a_translated_sphere_with_a_ray() {
    let r =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut s = Sphere::new();
    s.transform = Matrix4::translation(5.0, 0.0, 0.0);
    let xs = r.intersect(s);
    assert_eq!(xs.len(), 0);
}

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub t: f32,
    pub object: Sphere,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        (self.t - other.t).abs() < EPSILON && self.object == other.object
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
        Intersection { t, object }
    }
}

pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
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
    let h = hit(xs);
    assert!(h.is_some());
    assert_eq!(h.unwrap().t, 1.0);
}

#[test]
fn test_the_hit_when_some_intersections_have_negative_t() {
    let s = Sphere::new();
    let xs = s.intersections(vec![1.0, -1.0]);
    let h = hit(xs);
    assert!(h.is_some());
    assert_eq!(h.unwrap().t, 1.0);
}

#[test]
fn test_the_hit_when_all_intersections_have_negative_t() {
    let s = Sphere::new();
    let xs = s.intersections(vec![-2.0, -1.0]);
    let h = hit(xs);
    assert!(h.is_none());
}

#[test]
fn test_the_hit_is_always_the_lowest_non_negative_intersection() {
    let s = Sphere::new();
    let xs = s.intersections(vec![5.0, 7.0, -3.0, 2.0]);
    let h = hit(xs);
    assert!(h.is_some());
    assert_eq!(h.unwrap().t, 2.0);
}

pub fn normal_at(sphere: Sphere, point: Tuple) -> Tuple {
    let object_point = sphere.transform.inverse() * point;
    let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
    let mut world_normal =
        sphere.transform.inverse().transpose() * object_normal;
    world_normal.w = 0.0;
    world_normal.normalize()
}

#[test]
fn test_the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
    let s = Sphere::new();
    let n = normal_at(s, Tuple::point(1.0, 0.0, 0.0));
    assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
}

#[test]
fn test_the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
    let s = Sphere::new();
    let n = normal_at(s, Tuple::point(0.0, 1.0, 0.0));
    assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
}

#[test]
fn test_the_normal_on_a_sphere_at_a_point_onn_the_z_axis() {
    let s = Sphere::new();
    let n = normal_at(s, Tuple::point(0.0, 0.0, 1.0));
    assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));
}

#[test]
fn test_the_normal_on_a_sphere_at_a_non_axial_point() {
    let s = Sphere::new();
    let n = normal_at(
        s,
        Tuple::point(3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0),
    );
    assert_eq!(
        n,
        Tuple::vector(3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0)
    )
}

#[test]
fn test_the_normal_is_a_normalized_vector() {
    let s = Sphere::new();
    let n = normal_at(
        s,
        Tuple::point(3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0),
    );
    assert_eq!(n, n.normalize());
}

#[test]
fn test_computing_the_normal_on_a_translated_sphere() {
    let mut s = Sphere::new();
    s.transform = Matrix4::translation(0.0, 1.0, 0.0);
    let n = normal_at(s, Tuple::point(0.0, 1.70711, -0.70711));
    assert_eq!(n, Tuple::vector(0.0, 0.70711, -0.70711))
}

#[test]
fn test_computing_the_normal_on_a_scaled_sphere() {
    let mut s = Sphere::new();
    s.transform = Matrix4::scaling(1.0, 0.5, 1.0);
    let n =
        normal_at(s, Tuple::point(0.0, 2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0));
    assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254));
}
