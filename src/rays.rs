use intersections::Intersection;
use materials::Material;
use matrices::Matrix4;
use spheres::Sphere;
use tuples::Tuple;

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

pub fn reflect(in_v: Tuple, normal: Tuple) -> Tuple {
    in_v - normal * 2.0 * in_v.dot(normal)
}

#[test]
fn test_reflecting_a_vector_approaching_at_45_degrees() {
    let v = Tuple::vector(1.0, -1.0, 0.0);
    let n = Tuple::vector(0.0, 1.0, 0.0);
    let r = reflect(v, n);
    assert_eq!(r, Tuple::vector(1.0, 1.0, 0.0));
}

#[test]
fn test_reflecting_a_vector_off_a_slanted_surface() {
    let v = Tuple::vector(0.0, -1.0, 0.0);
    let n = Tuple::vector(2f32.sqrt() / 2.0, 2f32.sqrt() / 2.0, 0.0);
    let r = reflect(v, n);
    assert_eq!(r, Tuple::vector(1.0, 0.0, 0.0));
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Tuple,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Tuple) -> Self {
        PointLight {
            position,
            intensity,
        }
    }
}

#[test]
fn test_a_point_light_has_a_position_and_intensity() {
    let intensity = Tuple::color(1.0, 1.0, 1.0);
    let position = Tuple::point(0.0, 0.0, 0.0);
    let light = PointLight::new(position, intensity);
    assert_eq!(light.intensity, intensity);
    assert_eq!(light.position, position);
}

pub fn lighting(
    material: Material,
    light: PointLight,
    point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
) -> Tuple {
    let black = Tuple::color(0.0, 0.0, 0.0);
    let diffuse;
    let specular;
    let effective_color = material.color * light.intensity;
    let lightv = (light.position - point).normalize();
    let ambient = effective_color * material.ambient;
    let light_dot_normal = lightv.dot(normalv);
    if light_dot_normal < 0.0 {
        diffuse = black;
        specular = black;
    } else {
        diffuse = effective_color * material.diffuse * light_dot_normal;
        let reflectv = reflect(-lightv, normalv);
        let reflect_dot_eye = reflectv.dot(eyev).powf(material.shininess);
        specular = if reflect_dot_eye <= 0.0 {
            black
        } else {
            light.intensity * material.specular * reflect_dot_eye
        };
    }
    ambient + diffuse + specular
}

#[test]
fn test_lighting_with_the_eye_between_the_light_and_the_surface() {
    let m = Material::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 0.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let result = lighting(m, light, position, eyev, normalv);
    assert_eq!(result, Tuple::color(1.9, 1.9, 1.9));
}

#[test]
fn test_lighting_with_the_eye_between_light_and_surface_eye_offset_45_degrees()
{
    let m = Material::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eyev = Tuple::vector(0.0, 2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 0.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let result = lighting(m, light, position, eyev, normalv);
    assert_eq!(result, Tuple::color(1.0, 1.0, 1.0));
}

#[test]
fn test_lighting_with_eye_opposite_surface_light_offset_45_degrees() {
    let m = Material::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 10.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let result = lighting(m, light, position, eyev, normalv);
    assert_eq!(result, Tuple::color(0.7364, 0.7364, 0.7364));
}

#[test]
fn test_lighting_with_eye_in_the_path_of_the_reflection_vector() {
    let m = Material::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eyev = Tuple::vector(0.0, -2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 10.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let result = lighting(m, light, position, eyev, normalv);
    assert_eq!(result, Tuple::color(1.63638, 1.63638, 1.63638));
}

#[test]
fn test_lighting_with_the_light_behind_the_surface() {
    let m = Material::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 0.0, 10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let result = lighting(m, light, position, eyev, normalv);
    assert_eq!(result, Tuple::color(0.1, 0.1, 0.1));
}
