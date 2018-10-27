use intersections::Intersection;
use materials::Material;
use matrices::Matrix4;
#[cfg(test)]
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
