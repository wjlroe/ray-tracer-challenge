use materials::Material;
use patterns::pattern_at_shape;
use shapes::Shape;
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
    object: Shape,
    light: PointLight,
    point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
    in_shadow: bool,
) -> Tuple {
    let black = Tuple::color(0.0, 0.0, 0.0);
    let diffuse;
    let specular;
    let color = material
        .pattern
        .map(|pattern| pattern_at_shape(pattern, object, point))
        .unwrap_or(material.color);
    let effective_color = color * light.intensity;
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
    if in_shadow {
        ambient
    } else {
        ambient + diffuse + specular
    }
}

#[test]
fn test_lighting_with_the_eye_between_the_light_and_the_surface() {
    let object = Shape::default();
    let m = Material::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 0.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let in_shadow = false;
    let result = lighting(m, object, light, position, eyev, normalv, in_shadow);
    assert_eq!(result, Tuple::color(1.9, 1.9, 1.9));
}

#[test]
fn test_lighting_with_the_eye_between_light_and_surface_eye_offset_45_degrees()
{
    let object = Shape::default();
    let m = Material::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eyev = Tuple::vector(0.0, 2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 0.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let in_shadow = false;
    let result = lighting(m, object, light, position, eyev, normalv, in_shadow);
    assert_eq!(result, Tuple::color(1.0, 1.0, 1.0));
}

#[test]
fn test_lighting_with_eye_opposite_surface_light_offset_45_degrees() {
    let object = Shape::default();
    let m = Material::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 10.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let in_shadow = false;
    let result = lighting(m, object, light, position, eyev, normalv, in_shadow);
    assert_eq!(result, Tuple::color(0.7364, 0.7364, 0.7364));
}

#[test]
fn test_lighting_with_eye_in_the_path_of_the_reflection_vector() {
    let object = Shape::default();
    let m = Material::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eyev = Tuple::vector(0.0, -2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 10.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let in_shadow = false;
    let result = lighting(m, object, light, position, eyev, normalv, in_shadow);
    assert_eq!(result, Tuple::color(1.63638, 1.63638, 1.63638));
}

#[test]
fn test_lighting_with_the_light_behind_the_surface() {
    let object = Shape::default();
    let m = Material::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 0.0, 10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let in_shadow = false;
    let result = lighting(m, object, light, position, eyev, normalv, in_shadow);
    assert_eq!(result, Tuple::color(0.1, 0.1, 0.1));
}

#[test]
fn test_lighting_with_the_surface_in_shadow() {
    let object = Shape::default();
    let m = Material::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 0.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let in_shadow = true;
    let result = lighting(m, object, light, position, eyev, normalv, in_shadow);
    assert_eq!(result, Tuple::color(0.1, 0.1, 0.1));
}

#[test]
fn test_lighting_with_a_pattern_applied() {
    // FIXME: we can remove this when parent module imports this...
    use patterns::Pattern;

    let object = Shape::default();
    let mut m = Material::default();
    m.pattern = Some(Pattern::stripe(
        Tuple::color(1.0, 1.0, 1.0),
        Tuple::color(0.0, 0.0, 0.0),
    ));
    m.ambient = 1.0;
    m.diffuse = 0.0;
    m.specular = 0.0;
    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 0.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let c1 = lighting(
        m,
        object,
        light,
        Tuple::point(0.9, 0.0, 0.0),
        eyev,
        normalv,
        false,
    );
    let c2 = lighting(
        m,
        object,
        light,
        Tuple::point(1.0, 0.0, 0.0),
        eyev,
        normalv,
        false,
    );
    assert_eq!(c1, Tuple::color(1.0, 1.0, 1.0));
    assert_eq!(c2, Tuple::color(0.0, 0.0, 0.0));
}
