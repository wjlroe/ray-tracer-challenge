use intersections::{find_hit, Intersection};
use lighting::PointLight;
use matrices::Matrix4;
use rays::Ray;
use shapes::{Shape, Sphere};
use tuples::Tuple;

pub struct World {
    pub light_source: Option<PointLight>,
    pub objects: Vec<Shape>,
}

impl World {
    pub fn new() -> Self {
        World {
            light_source: None,
            objects: vec![],
        }
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.objects.push(shape);
    }

    pub fn intersect_world(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = self
            .objects
            .iter()
            .flat_map(|object| object.intersect(ray))
            .collect::<Vec<Intersection>>();
        intersections.sort_unstable();
        intersections
    }

    pub fn color_at(&self, ray: &Ray, remaining: i32) -> Tuple {
        let xs = self.intersect_world(ray);
        if let Some(mut hit) = find_hit(&xs) {
            hit.prepare_hit(ray);
            hit.shade_hit(self, remaining) // .normalize()
        } else {
            Tuple::color(0.0, 0.0, 0.0)
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        if let Some(light) = self.light_source {
            let v = light.position - point;
            let distance = v.magnitude();
            let ray = Ray::new(point, v.normalize());
            let xs = self.intersect_world(&ray);
            if let Some(hit) = find_hit(&xs) {
                hit.t < distance
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
use REFLECTION_RECURSION_LIMIT;

#[test]
fn test_creating_a_world() {
    let w = World::new();
    assert_eq!(w.objects.len(), 0);
    assert!(w.light_source.is_none());
}

#[test]
fn test_intersect_a_world_with_a_ray() {
    let world = World::default();
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let xs = world.intersect_world(&ray);
    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 4.5);
    assert_eq!(xs[2].t, 5.5);
    assert_eq!(xs[3].t, 6.0);
}

#[test]
fn test_the_color_a_ray_misses() {
    let w = World::default();
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
    let c = w.color_at(&ray, REFLECTION_RECURSION_LIMIT);
    assert_eq!(c, Tuple::color(0.0, 0.0, 0.0));
}

#[test]
fn test_the_color_when_a_ray_hits() {
    let w = World::default();
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let c = w.color_at(&ray, REFLECTION_RECURSION_LIMIT);
    assert_eq!(c, Tuple::color(0.38066, 0.47583, 0.2855));
}

#[test]
fn test_the_color_with_an_intersection_behind_the_ray() {
    let mut world = World::default();
    world.objects[0].material.ambient = 1.0;
    world.objects[1].material.ambient = 1.0;
    assert_eq!(world.objects[0].material.ambient, 1.0);
    let ray =
        Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
    let c = world.color_at(&ray, REFLECTION_RECURSION_LIMIT);
    assert_eq!(c, world.objects[1].material.color);
}

#[test]
fn test_color_at_with_mutually_reflective_surfaces() {
    use shapes::Plane;

    let mut world = World::default();
    world.light_source = Some(PointLight::new(
        Tuple::point(0.0, 0.0, 0.0),
        Tuple::color(1.0, 1.0, 1.0),
    ));

    let mut lower = Plane::new();
    lower.material.reflective = 1.0;
    lower.transform = Matrix4::translation(0.0, -1.0, 0.0);
    world.add_shape(lower);

    let mut upper = Plane::new();
    upper.material.reflective = 1.0;
    upper.transform = Matrix4::translation(0.0, 1.0, 0.0);
    world.add_shape(upper);

    let ray =
        Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
    let _ = world.color_at(&ray, REFLECTION_RECURSION_LIMIT);
    assert!(true); // We're testing color_at terminates and gets here
}

#[test]
fn test_there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
    let world = World::default();
    let point = Tuple::point(0.0, 10.0, 0.0);
    assert!(!world.is_shadowed(point));
}

#[test]
fn test_shadow_when_an_object_is_between_the_point_and_the_light() {
    let world = World::default();
    let point = Tuple::point(10.0, -10.0, 10.0);
    assert!(world.is_shadowed(point));
}

#[test]
fn test_there_is_no_shadow_when_an_object_is_behind_the_light() {
    let world = World::default();
    let point = Tuple::point(-20.0, 20.0, -20.0);
    assert!(!world.is_shadowed(point));
}

#[test]
fn test_there_is_no_shadow_when_an_object_is_behind_the_point() {
    let world = World::default();
    let point = Tuple::point(-2.0, 2.0, -2.0);
    assert!(!world.is_shadowed(point));
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
