use std::ops;

const EPSILON: f32 = 0.00001;

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Tuple::new(x, y, z, 1.0)
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2))
            .sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let magnitude = self.magnitude();
        Tuple {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        }
    }

    pub fn dot(&self, other: Tuple) -> f32 {
        self.x * other.x
            + self.y * other.y
            + self.z * other.z
            + self.w * other.w
    }

    pub fn cross(&self, other: Tuple) -> Tuple {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[test]
fn test_w_of_1_is_a_point() {
    let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 1.0);
    assert!(a.is_point());
    assert!(!a.is_vector());
}

#[test]
fn test_w_of_0_is_a_vector() {
    let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 0.0);
    assert!(!a.is_point());
    assert!(a.is_vector());
}

#[test]
fn test_point_factory() {
    let p = Tuple::point(4.0, -4.0, 3.0);
    assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0));
}

#[test]
fn test_vector_factory() {
    let v = Tuple::vector(4.0, -4.0, 3.0);
    assert_eq!(v, Tuple::new(4.0, -4.0, 3.0, 0.0));
}

#[test]
fn test_magnitude_of_vectors() {
    {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    {
        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    {
        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14f32.sqrt());
    }

    {
        let v = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14f32.sqrt());
    }
}

#[test]
fn test_normalizing_vectors() {
    {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0));
    }

    {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.normalize(), Tuple::vector(0.26726, 0.53452, 0.80178));
    }

    {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        assert!((norm.magnitude() - 1.0).abs() < EPSILON);
    }
}

#[test]
fn test_dot_product_of_two_vectors() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(2.0, 3.0, 4.0);
    assert!((a.dot(b) - 20.0).abs() < EPSILON);
}

#[test]
fn test_cross_product_of_two_vectors() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(2.0, 3.0, 4.0);
    assert_eq!(a.cross(b), Tuple::vector(-1.0, 2.0, -1.0));
    assert_eq!(b.cross(a), Tuple::vector(1.0, -2.0, 1.0));
}

impl ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

#[test]
fn test_adding_two_tuples() {
    let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
    let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
    assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
}

impl ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

#[test]
fn test_subtracting_two_points() {
    let p1 = Tuple::point(3.0, 2.0, 1.0);
    let p2 = Tuple::point(5.0, 6.0, 7.0);
    assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
}

#[test]
fn test_subtracting_a_vector_from_a_point() {
    let p = Tuple::point(3.0, 2.0, 1.0);
    let v = Tuple::vector(5.0, 6.0, 7.0);
    assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
}

#[test]
fn test_subtracting_two_vectors() {
    let v1 = Tuple::vector(3.0, 2.0, 1.0);
    let v2 = Tuple::vector(5.0, 6.0, 7.0);
    assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
}

#[test]
fn test_subtracting_a_vector_from_the_zero_vector() {
    let zero = Tuple::vector(0.0, 0.0, 0.0);
    let v = Tuple::vector(1.0, -2.0, 3.0);
    assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0));
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

#[test]
fn test_negating_a_tuple() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
}

impl ops::Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, other: f32) -> Tuple {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

#[test]
fn test_multiplying_a_tuple_by_a_scalar() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
}

#[test]
fn test_multiplying_a_tuple_by_a_fraction() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
}

impl ops::Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, other: f32) -> Tuple {
        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

#[test]
fn test_dividing_a_tuple_by_a_scalar() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
            && (self.w - other.w).abs() < EPSILON
    }
}
