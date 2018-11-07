use materials::Material;
use tuples::Tuple;

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
        let reflectv = -lightv.reflect(normalv);
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
