use num::{Float, One, Signed, Zero};
use std::fmt;
use std::ops;

const EPSILON: f32 = 0.00001;

pub type T = Float + fmt::Display;

#[derive(Copy, Clone)]
pub struct Tuple<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Tuple<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn point(x: T, y: T, z: T) -> Self {
        Tuple::new(x, y, z, One::one())
    }

    pub fn vector(x: T, y: T, z: T) -> Self {
        Tuple::new(x, y, z, Zero::zero())
    }

    pub fn color(red: T, green: T, blue: T) -> Self {
        Tuple::new(red, green, blue, Zero::zero())
    }

    pub fn red(&self) -> T {
        self.x
    }

    pub fn green(&self) -> T {
        self.y
    }

    pub fn blue(&self) -> T {
        self.z
    }

    pub fn is_point(&self) -> bool {
        self.w == One::one()
    }

    pub fn is_vector(&self) -> bool {
        self.w == Zero::zero()
    }

    pub fn magnitude(&self) -> T {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2))
            .sqrt()
    }

    pub fn normalize(&self) -> Tuple<T> {
        let magnitude = self.magnitude();
        Tuple {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        }
    }

    pub fn dot(&self, other: Tuple<T>) -> T {
        self.x * other.x
            + self.y * other.y
            + self.z * other.z
            + self.w * other.w
    }

    pub fn cross(&self, other: Tuple<T>) -> Tuple<T> {
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
fn test_new_color() {
    let color = Tuple::color(-0.5, 0.4, 1.7);
    assert_eq!(color.red(), -0.5);
    assert_eq!(color.green(), 0.4);
    assert_eq!(color.blue(), 1.7);
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

impl<T> ops::Add for Tuple<T> {
    type Output = Tuple<T>;

    fn add(self, other: Tuple<T>) -> Tuple<T> {
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

#[test]
fn test_adding_colors() {
    let c1 = Tuple::color(0.9, 0.6, 0.75);
    let c2 = Tuple::color(0.7, 0.1, 0.25);
    assert_eq!(c1 + c2, Tuple::color(1.6, 0.7, 1.0));
}

impl<T> ops::Sub for Tuple<T> {
    type Output = Tuple<T>;

    fn sub(self, other: Tuple<T>) -> Tuple<T> {
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

#[test]
fn test_subtracting_colors() {
    let c1 = Tuple::color(0.9, 0.6, 0.75);
    let c2 = Tuple::color(0.7, 0.1, 0.25);
    assert_eq!(c1 - c2, Tuple::color(0.2, 0.5, 0.5))
}

impl<T> ops::Neg for Tuple<T> {
    type Output = Tuple<T>;

    fn neg(self) -> Tuple<T> {
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

impl<T> ops::Mul<T> for Tuple<T> {
    type Output = Tuple<T>;

    fn mul(self, other: T) -> Tuple<T> {
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

#[test]
fn test_multiplying_a_color_by_a_scalar() {
    let c = Tuple::color(0.2, 0.3, 0.4);
    assert_eq!(c * 2.0, Tuple::color(0.4, 0.6, 0.8));
}

impl<T> ops::Mul for Tuple<T> {
    type Output = Tuple<T>;

    fn mul(self, other: Tuple<T>) -> Tuple<T> {
        Tuple {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

#[test]
fn test_multiplying_colors() {
    let c1 = Tuple::color(1.0, 0.2, 0.4);
    let c2 = Tuple::color(0.9, 1.0, 0.1);
    assert_eq!(c1 * c2, Tuple::color(0.9, 0.2, 0.04));
}

impl<T> ops::Div<T> for Tuple<T> {
    type Output = Tuple<T>;

    fn div(self, other: T) -> Tuple<T> {
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

impl<T> PartialEq for Tuple<T> {
    fn eq(&self, other: &Tuple<T>) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
            && (self.w - other.w).abs() < EPSILON
    }
}

impl<T> fmt::Debug for Tuple<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}
