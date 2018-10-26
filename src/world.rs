use matrices::Matrix4;
use rays::{Intersection, PointLight, Ray, Sphere};
use tuples::Tuple;

pub struct World {
    pub light_source: Option<PointLight>,
    pub objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        World {
            light_source: None,
            objects: vec![],
        }
    }
}

#[test]
fn test_creating_a_world() {
    let w = World::new();
    assert_eq!(w.objects.len(), 0);
    assert!(w.light_source.is_none());
}

impl Default for World {
    fn default() -> Self {
        let mut sphere1 = Sphere::new();
        sphere1.material.color = Tuple::color(0.8, 1.0, 0.6);
        sphere1.material.diffuse = 0.7;
        sphere1.material.specular = 0.2;
        let mut sphere2 = Sphere::new();
        sphere2.transform = Matrix4::scaling(0.5, 0.5, 0.5);
        let light = PointLight::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Tuple::color(1.0, 1.0, 1.0),
        );
        World {
            light_source: Some(light),
            objects: vec![sphere1, sphere2],
        }
    }
}

#[test]
fn test_the_default_world() {
    let world = World::default();
    let light = PointLight::new(
        Tuple::point(-10.0, 10.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let mut s1 = Sphere::new();
    s1.material.color = Tuple::color(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    let mut s2 = Sphere::new();
    s2.transform = Matrix4::scaling(0.5, 0.5, 0.5);
    assert_eq!(world.light_source, Some(light));
    assert!(world.objects.contains(&s1));
    assert!(world.objects.contains(&s2));
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
