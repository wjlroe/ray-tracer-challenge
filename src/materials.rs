use super::float_eq;
use patterns::Pattern;
use tuples::Tuple;

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub color: Tuple,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub pattern: Option<Pattern>,
    pub reflective: f32,
}

impl Material {
    pub fn new(
        color: Tuple,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
        reflective: f32,
    ) -> Self {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            pattern: None,
            reflective,
        }
    }
}

impl Eq for Material {}

impl PartialEq for Material {
    fn eq(&self, other: &Material) -> bool {
        self.color == other.color
            && float_eq(self.ambient, other.ambient)
            && float_eq(self.diffuse, other.diffuse)
            && float_eq(self.specular, other.specular)
            && float_eq(self.shininess, other.shininess)
            && self.pattern == other.pattern
            && float_eq(self.reflective, other.reflective)
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Tuple::color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
            reflective: 0.0,
        }
    }
}

#[test]
fn test_the_default_material() {
    let m = Material::default();
    assert_eq!(m.color, Tuple::color(1.0, 1.0, 1.0));
    assert_eq!(m.ambient, 0.1);
    assert_eq!(m.diffuse, 0.9);
    assert_eq!(m.specular, 0.9);
    assert_eq!(m.shininess, 200.0);
    assert_eq!(m.pattern, None);
    assert_eq!(m.reflective, 0.0);
}
