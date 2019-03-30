use matrices::Matrix4;
use shapes::Shape;
use tuples::Tuple;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pattern {
    pub a: Tuple,
    pub b: Tuple,
    pub transform: Matrix4,
}

impl Pattern {
    pub fn stripe(a: Tuple, b: Tuple) -> Self {
        Pattern {
            a,
            b,
            transform: Matrix4::default(),
        }
    }

    pub fn stripe_at(&self, point: Tuple) -> Tuple {
        if point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

pub fn stripe_at_object(
    pattern: Pattern,
    object: Shape,
    point: Tuple,
) -> Tuple {
    let object_space = object.transform.inverse() * point;
    let pattern_space = pattern.transform.inverse() * object_space;
    pattern.stripe_at(pattern_space)
}

#[cfg(test)]
mod tests {
    use matrices::Matrix4;
    use patterns::{stripe_at_object, Pattern};
    use shapes::Shape;
    use tuples::Tuple;

    fn white() -> Tuple {
        Tuple::color(1.0, 1.0, 1.0)
    }

    fn black() -> Tuple {
        Tuple::color(0.0, 0.0, 0.0)
    }

    #[test]
    fn test_creating_a_stripe_pattern() {
        let pattern = Pattern::stripe(white(), black());
        assert_eq!(pattern.a, white());
        assert_eq!(pattern.b, black());
    }

    #[test]
    fn test_a_stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::stripe(white(), black());
        assert_eq!(pattern.stripe_at(Tuple::point(0.0, 0.0, 0.0)), white());
        assert_eq!(pattern.stripe_at(Tuple::point(0.0, 1.0, 0.0)), white());
        assert_eq!(pattern.stripe_at(Tuple::point(0.0, 2.0, 0.0)), white());
    }

    #[test]
    fn test_a_stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::stripe(white(), black());
        assert_eq!(pattern.stripe_at(Tuple::point(0.0, 0.0, 0.0)), white());
        assert_eq!(pattern.stripe_at(Tuple::point(0.0, 0.0, 1.0)), white());
        assert_eq!(pattern.stripe_at(Tuple::point(0.0, 0.0, 2.0)), white());
    }

    #[test]
    fn test_a_stripe_pattern_alternates_in_x() {
        let pattern = Pattern::stripe(white(), black());
        assert_eq!(pattern.stripe_at(Tuple::point(0.0, 0.0, 0.0)), white());
        assert_eq!(pattern.stripe_at(Tuple::point(0.9, 0.0, 0.0)), white());
        assert_eq!(pattern.stripe_at(Tuple::point(1.0, 0.0, 0.0)), black());
        assert_eq!(pattern.stripe_at(Tuple::point(-0.1, 0.0, 0.0)), black());
        assert_eq!(pattern.stripe_at(Tuple::point(-1.0, 0.0, 0.0)), black());
        assert_eq!(pattern.stripe_at(Tuple::point(-1.1, 0.0, 0.0)), white());
    }

    #[test]
    fn test_stripes_with_an_object_transform() {
        let mut object = Shape::default();
        object.transform = Matrix4::scaling(2.0, 2.0, 2.0);
        let pattern = Pattern::stripe(white(), black());
        assert_eq!(
            stripe_at_object(pattern, object, Tuple::point(1.5, 0.0, 0.0)),
            white()
        );
    }

    #[test]
    fn test_stripes_with_a_pattern_transformation() {
        let object = Shape::default();
        let mut pattern = Pattern::stripe(white(), black());
        pattern.transform = Matrix4::scaling(2.0, 2.0, 2.0);
        assert_eq!(
            stripe_at_object(pattern, object, Tuple::point(1.5, 0.0, 0.0)),
            white()
        );
    }

    #[test]
    fn test_stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object = Shape::default();
        object.transform = Matrix4::scaling(2.0, 2.0, 2.0);
        let mut pattern = Pattern::stripe(white(), black());
        pattern.transform = Matrix4::translation(0.5, 0.0, 0.0);
        assert_eq!(
            stripe_at_object(pattern, object, Tuple::point(2.5, 0.0, 0.0)),
            white()
        );
    }
}
