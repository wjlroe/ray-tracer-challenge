use intersections::Intersection;
use materials::Material;
use matrices::Matrix4;
use tuples::Tuple;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Sphere {
    pub transform: Matrix4,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transform: Matrix4::default(),
            material: Material::default(),
        }
    }

    pub fn intersections(&self, ts: Vec<f32>) -> Vec<Intersection> {
        ts.iter()
            .map(|t| Intersection::new(*t, self.clone()))
            .collect::<Vec<_>>()
    }

    pub fn normal_at(&self, point: Tuple) -> Tuple {
        let object_point = self.transform.inverse() * point;
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
        let mut world_normal =
            self.transform.inverse().transpose() * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
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
    let m = Material::new(Tuple::color(2.0, 0.0, 5.0), 2.0, 3.0, 4.0, 5.0);
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
