use tuples::Tuple;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pattern {
    pub a: Tuple,
    pub b: Tuple,
}

impl Pattern {
    pub fn stripe(a: Tuple, b: Tuple) -> Self {
        Pattern { a, b }
    }

    pub fn stripe_at(&self, point: Tuple) -> Tuple {
        if point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use patterns::Pattern;
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
}
