use matrices::Matrix4;
use shapes::Shape;
use tuples::Tuple;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PatternKind {
    Stripe(Tuple, Tuple),
    Gradient(Tuple, Tuple),
    Ring(Tuple, Tuple),
    TestPattern,
}

impl Default for PatternKind {
    fn default() -> PatternKind {
        PatternKind::Stripe(Tuple::default(), Tuple::default())
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Pattern {
    pub transform: Matrix4,
    pub kind: PatternKind,
}

impl Pattern {
    pub fn stripe(a: Tuple, b: Tuple) -> Self {
        Self {
            transform: Matrix4::default(),
            kind: PatternKind::Stripe(a, b),
        }
    }

    pub fn gradient(a: Tuple, b: Tuple) -> Self {
        Self {
            transform: Matrix4::default(),
            kind: PatternKind::Gradient(a, b),
        }
    }

    pub fn ring(a: Tuple, b: Tuple) -> Self {
        Self {
            transform: Matrix4::default(),
            kind: PatternKind::Ring(a, b),
        }
    }

    fn color_at(&self, point: Tuple) -> Tuple {
        match self.kind {
            PatternKind::Stripe(a, b) => {
                if point.x.floor() % 2.0 == 0.0 {
                    a
                } else {
                    b
                }
            }
            PatternKind::Gradient(a, b) => {
                a + (b - a) * (point.x - point.x.floor())
            }
            PatternKind::Ring(a, b) => {
                if (point.x.powf(2.0) + point.z.powf(2.0)).sqrt().floor() % 2.0
                    == 0.0
                {
                    a
                } else {
                    b
                }
            }
            PatternKind::TestPattern => Tuple::color(point.x, point.y, point.z),
        }
    }
}

pub fn pattern_at_shape(
    pattern: Pattern,
    object: Shape,
    point: Tuple,
) -> Tuple {
    let object_space = object.transform.inverse() * point;
    let pattern_space = pattern.transform.inverse() * object_space;
    pattern.color_at(pattern_space)
}

#[cfg(test)]
mod tests {
    use matrices::Matrix4;
    use patterns::{pattern_at_shape, Pattern, PatternKind};
    use shapes::Shape;
    use tuples::Tuple;

    fn white() -> Tuple {
        Tuple::color(1.0, 1.0, 1.0)
    }

    fn black() -> Tuple {
        Tuple::color(0.0, 0.0, 0.0)
    }

    fn test_pattern() -> Pattern {
        Pattern {
            kind: PatternKind::TestPattern,
            ..Pattern::default()
        }
    }

    #[test]
    fn test_creating_a_stripe_pattern() {
        let pattern = Pattern::stripe(white(), black());
        match pattern.kind {
            PatternKind::Stripe(a, b) => {
                assert_eq!(a, white());
                assert_eq!(b, black());
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_a_stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::stripe(white(), black());
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.0)), white());
        assert_eq!(pattern.color_at(Tuple::point(0.0, 1.0, 0.0)), white());
        assert_eq!(pattern.color_at(Tuple::point(0.0, 2.0, 0.0)), white());
    }

    #[test]
    fn test_a_stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::stripe(white(), black());
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.0)), white());
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 1.0)), white());
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 2.0)), white());
    }

    #[test]
    fn test_a_stripe_pattern_alternates_in_x() {
        let pattern = Pattern::stripe(white(), black());
        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.0)), white());
        assert_eq!(pattern.color_at(Tuple::point(0.9, 0.0, 0.0)), white());
        assert_eq!(pattern.color_at(Tuple::point(1.0, 0.0, 0.0)), black());
        assert_eq!(pattern.color_at(Tuple::point(-0.1, 0.0, 0.0)), black());
        assert_eq!(pattern.color_at(Tuple::point(-1.0, 0.0, 0.0)), black());
        assert_eq!(pattern.color_at(Tuple::point(-1.1, 0.0, 0.0)), white());
    }

    #[test]
    fn test_a_pattern_with_an_object_transform() {
        let mut shape = Shape::default();
        shape.transform = Matrix4::scaling(2.0, 2.0, 2.0);
        let pattern = test_pattern();
        assert_eq!(
            pattern_at_shape(pattern, shape, Tuple::point(2.0, 3.0, 4.0)),
            Tuple::color(1.0, 1.5, 2.0)
        );
    }

    #[test]
    fn test_a_pattern_with_a_pattern_transformation() {
        let shape = Shape::default();
        let mut pattern = test_pattern();
        pattern.transform = Matrix4::scaling(2.0, 2.0, 2.0);
        assert_eq!(
            pattern_at_shape(pattern, shape, Tuple::point(2.0, 3.0, 4.0)),
            Tuple::color(1.0, 1.5, 2.0)
        );
    }

    #[test]
    fn test_a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let mut shape = Shape::default();
        shape.transform = Matrix4::scaling(2.0, 2.0, 2.0);
        let mut pattern = test_pattern();
        pattern.transform = Matrix4::translation(0.5, 1.0, 1.5);
        assert_eq!(
            pattern_at_shape(pattern, shape, Tuple::point(2.5, 3.0, 3.5)),
            Tuple::color(0.75, 0.5, 0.25)
        );
    }

    #[test]
    fn test_default_pattern_transform() {
        let stripe = Pattern::default();
        assert_eq!(Matrix4::default(), stripe.transform);
    }

    #[test]
    fn test_assigning_a_transformation() {
        let mut stripe = Pattern::default();
        stripe.transform = Matrix4::translation(1.0, 2.0, 3.0);
        assert_eq!(Matrix4::translation(1.0, 2.0, 3.0), stripe.transform);
    }

    #[test]
    fn test_a_gradient_linearly_interpolates_between_colors() {
        let pattern = Pattern::gradient(white(), black());
        assert_eq!(white(), pattern.color_at(Tuple::point(0.0, 0.0, 0.0)));
        assert_eq!(
            Tuple::color(0.75, 0.75, 0.75),
            pattern.color_at(Tuple::point(0.25, 0.0, 0.0))
        );
        assert_eq!(
            Tuple::color(0.5, 0.5, 0.5),
            pattern.color_at(Tuple::point(0.5, 0.0, 0.0))
        );
        assert_eq!(
            Tuple::color(0.25, 0.25, 0.25),
            pattern.color_at(Tuple::point(0.75, 0.0, 0.0))
        );
    }

    #[test]
    fn test_a_ring_should_extend_in_both_x_and_z() {
        let pattern = Pattern::ring(white(), black());
        assert_eq!(white(), pattern.color_at(Tuple::point(0.0, 0.0, 0.0)));
        assert_eq!(black(), pattern.color_at(Tuple::point(1.0, 0.0, 0.0)));
        assert_eq!(black(), pattern.color_at(Tuple::point(0.0, 0.0, 1.0)));
        assert_eq!(black(), pattern.color_at(Tuple::point(0.708, 0.0, 0.708)));
    }
}
