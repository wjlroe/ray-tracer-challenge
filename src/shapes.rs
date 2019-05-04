use super::float_eq;
use intersections::Intersection;
use materials::Material;
use matrices::Matrix4;
use rays::Ray;
use tuples::Tuple;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ShapeKind {
    Sphere,
    Plane,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Shape {
    pub transform: Matrix4,
    pub material: Material,
    pub shape_kind: ShapeKind,
}

impl Shape {
    pub fn normal_at(&self, point: Tuple) -> Tuple {
        let local_point = self.transform.inverse() * point;
        let local_normal = self.local_normal_at(local_point);
        let mut world_normal =
            self.transform.inverse().transpose() * local_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        match self.shape_kind {
            ShapeKind::Plane => Tuple::vector(0.0, 1.0, 0.0),
            ShapeKind::Sphere => point - Tuple::point(0.0, 0.0, 0.0),
        }
    }

    pub fn intersections(&self, ts: Vec<f32>) -> Vec<Intersection> {
        ts.iter()
            .map(|t| Intersection::new(*t, self.clone()))
            .collect::<Vec<Intersection>>()
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(self.transform.inverse());
        self.local_intersect(local_ray)
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        match self.shape_kind {
            ShapeKind::Plane => {
                if float_eq(ray.direction.y, 0.0) {
                    vec![]
                } else {
                    let t = -ray.origin.y / ray.direction.y;
                    vec![Intersection::new(t, self.clone())]
                }
            }
            ShapeKind::Sphere => {
                let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);
                let a = ray.direction.dot(ray.direction);
                let b = 2.0 * ray.direction.dot(sphere_to_ray);
                let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
                let discriminant = b.powi(2) - 4.0 * a * c;
                if discriminant < 0.0 {
                    vec![]
                } else {
                    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
                    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
                    let i1 = Intersection::new(t1, self.clone());
                    let i2 = Intersection::new(t2, self.clone());
                    if t1 > t2 {
                        vec![i2, i1]
                    } else {
                        vec![i1, i2]
                    }
                }
            }
        }
    }
}

impl Default for Shape {
    fn default() -> Shape {
        Shape {
            transform: Matrix4::default(),
            material: Material::default(),
            shape_kind: ShapeKind::Sphere,
        }
    }
}

#[test]
fn test_the_default_transformation() {
    let s = Shape::default();
    assert_eq!(s.transform, Matrix4::default());
}

#[test]
fn test_assigning_a_transformation() {
    let mut s = Shape::default();
    s.transform = Matrix4::translation(2.0, 3.0, 4.0);
    assert_eq!(s.transform, Matrix4::translation(2.0, 3.0, 4.0));
}

#[test]
fn test_the_default_material() {
    let s = Shape::default();
    assert_eq!(s.material, Material::default());
}

#[test]
fn test_assigning_a_material() {
    let mut s = Shape::default();
    let mut m = Material::default();
    m.ambient = 1.0;
    s.material = m;
    assert_eq!(s.material, m);
}

pub struct Sphere {}

impl Sphere {
    pub fn new() -> Shape {
        Shape {
            transform: Matrix4::default(),
            material: Material::default(),
            shape_kind: ShapeKind::Sphere,
        }
    }
}

#[test]
fn test_a_spheres_default_transformation() {
    let s = Sphere::new();
    assert_eq!(s.transform, Matrix4::default());
}

#[test]
fn test_changing_a_spheres_transformation() {
    let mut s = Sphere::new();
    let t = Matrix4::translation(2.0, 3.0, 4.0);
    s.transform = t;
    assert_eq!(s.transform, t);
}

#[test]
fn test_a_sphere_has_a_default_material() {
    let s = Sphere::new();
    assert_eq!(s.material, Material::default());
}

#[test]
fn test_a_sphere_may_be_assigned_a_material() {
    let mut s = Sphere::new();
    let m = Material::new(Tuple::color(2.0, 0.0, 5.0), 2.0, 3.0, 4.0, 5.0, 0.0);
    s.material = m;
    assert_eq!(s.material, m);
}

#[test]
fn test_the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::point(1.0, 0.0, 0.0));
    assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
}

#[test]
fn test_the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::point(0.0, 1.0, 0.0));
    assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
}

#[test]
fn test_the_normal_on_a_sphere_at_a_point_onn_the_z_axis() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::point(0.0, 0.0, 1.0));
    assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));
}

#[test]
fn test_the_normal_on_a_sphere_at_a_non_axial_point() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::point(
        3f32.sqrt() / 3.0,
        3f32.sqrt() / 3.0,
        3f32.sqrt() / 3.0,
    ));
    assert_eq!(
        n,
        Tuple::vector(3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0)
    )
}

#[test]
fn test_the_normal_is_a_normalized_vector() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::point(
        3f32.sqrt() / 3.0,
        3f32.sqrt() / 3.0,
        3f32.sqrt() / 3.0,
    ));
    assert_eq!(n, n.normalize());
}

#[test]
fn test_computing_the_normal_on_a_translated_sphere() {
    let mut s = Sphere::new();
    s.transform = Matrix4::translation(0.0, 1.0, 0.0);
    let n = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));
    assert_eq!(n, Tuple::vector(0.0, 0.70711, -0.70711))
}

#[test]
fn test_computing_the_normal_on_a_scaled_sphere() {
    let mut s = Sphere::new();
    s.transform = Matrix4::scaling(1.0, 0.5, 1.0);
    let n =
        s.normal_at(Tuple::point(0.0, 2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0));
    assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254));
}

#[test]
fn test_a_ray_intersects_a_sphere_at_two_points() {
    let r =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 6.0);
}

#[test]
fn test_a_ray_intersects_a_sphere_at_a_tangent() {
    let r =
        Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 5.0);
    assert_eq!(xs[1].t, 5.0);
}

#[test]
fn test_a_ray_misses_a_sphere() {
    let r =
        Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 0);
}

#[test]
fn test_a_ray_originates_inside_a_sphere() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -1.0);
    assert_eq!(xs[1].t, 1.0);
}

#[test]
fn test_a_sphere_is_behind_a_ray() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -6.0);
    assert_eq!(xs[1].t, -4.0);
}

#[test]
fn test_intersect_sets_the_object_on_the_intersection() {
    let r =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].object, s.clone());
    assert_eq!(xs[1].object, s.clone());
}

#[test]
fn test_intersecting_a_scaled_sphere_with_a_ray() {
    let r =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut s = Sphere::new();
    s.transform = Matrix4::scaling(2.0, 2.0, 2.0);
    let xs = s.intersect(&r);
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
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 0);
}

pub struct Plane {}

impl Plane {
    pub fn new() -> Shape {
        let mut shape = Shape::default();
        shape.shape_kind = ShapeKind::Plane;
        shape
    }
}

#[test]
fn test_the_normal_of_a_plane_is_constant_everywhere() {
    let p = Plane::new();
    let normal = Tuple::vector(0.0, 1.0, 0.0);
    assert_eq!(p.local_normal_at(Tuple::point(0.0, 0.0, 0.0)), normal);
    assert_eq!(p.local_normal_at(Tuple::point(10.0, 0.0, -10.0)), normal);
    assert_eq!(p.local_normal_at(Tuple::point(-5.0, 0.0, 150.0)), normal);
}

#[test]
fn test_intersect_with_a_ray_parallel_to_the_plane() {
    let p = Plane::new();
    let r =
        Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let xs = p.local_intersect(r);
    assert!(xs.is_empty());
}

#[test]
fn test_intersect_with_a_coplanar_ray() {
    let p = Plane::new();
    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let xs = p.local_intersect(r);
    assert!(xs.is_empty());
}

#[test]
fn test_a_ray_intersecting_a_plane_from_above() {
    let p = Plane::new();
    let r =
        Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));
    let xs = p.local_intersect(r);
    assert_eq!(xs.len(), 1);
    assert_eq!(xs[0].t, 1.0);
    assert_eq!(xs[0].object, p);
}

#[test]
fn test_a_ray_intersecting_a_plane_from_below() {
    let p = Plane::new();
    let r =
        Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
    let xs = p.local_intersect(r);
    assert_eq!(xs.len(), 1);
    assert_eq!(xs[0].t, 1.0);
    assert_eq!(xs[0].object, p);
}
