use std::default::Default;
use std::fmt;
use std::ops;
use tuples::Tuple;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix2 {
    pub rows: [[f32; 2]; 2],
}

impl Matrix2 {
    pub fn from_rows(rows: [[f32; 2]; 2]) -> Self {
        Matrix2 { rows }
    }
}

#[test]
fn test_a_2x2_matrix_should_be_representable() {
    let matrix = Matrix2::from_rows([[-3.0, 5.0], [1.0, -2.0]]);
    assert_eq!(matrix.rows[0][0], -3.0);
    assert_eq!(matrix.rows[0][1], 5.0);
    assert_eq!(matrix.rows[1][0], 1.0);
    assert_eq!(matrix.rows[1][1], -2.0);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix3 {
    pub rows: [[f32; 3]; 3],
}

impl Matrix3 {
    pub fn from_rows(rows: [[f32; 3]; 3]) -> Self {
        Matrix3 { rows }
    }
}

#[test]
fn test_a_3x3_matrix_should_be_representable() {
    let matrix = Matrix3::from_rows([
        [-3.0, 5.0, 0.0],
        [1.0, -2.0, -7.0],
        [0.0, 1.0, 1.0],
    ]);
    assert_eq!(matrix.rows[0][0], -3.0);
    assert_eq!(matrix.rows[1][1], -2.0);
    assert_eq!(matrix.rows[2][2], 1.0);
}

#[derive(Copy, Clone, PartialEq)]
pub struct Matrix4 {
    pub rows: [[f32; 4]; 4],
}

impl fmt::Debug for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..4 {
            for col in 0..4 {
                write!(f, " | {:3.1}", self.rows[row][col])?;
            }
            write!(f, " |")?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl ops::Mul<Matrix4> for Matrix4 {
    type Output = Self;

    fn mul(self, other: Matrix4) -> Self {
        let mut rows = [[Default::default(); 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                rows[row][col] = self.rows[row][0] * other.rows[0][col]
                    + self.rows[row][1] * other.rows[1][col]
                    + self.rows[row][2] * other.rows[2][col]
                    + self.rows[row][3] * other.rows[3][col];
            }
        }
        Matrix4::from_rows(rows)
    }
}

#[test]
fn test_multiplying_two_matrices() {
    let matrix_a = Matrix4::from_rows([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 3.0, 4.0, 5.0],
        [3.0, 4.0, 5.0, 6.0],
        [4.0, 5.0, 6.0, 7.0],
    ]);
    let matrix_b = Matrix4::from_rows([
        [0.0, 1.0, 2.0, 4.0],
        [1.0, 2.0, 4.0, 8.0],
        [2.0, 4.0, 8.0, 16.0],
        [4.0, 8.0, 16.0, 32.0],
    ]);
    let expected = Matrix4::from_rows([
        [24.0, 49.0, 98.0, 196.0],
        [31.0, 64.0, 128.0, 256.0],
        [38.0, 79.0, 158.0, 316.0],
        [45.0, 94.0, 188.0, 376.0],
    ]);
    assert_eq!(matrix_a * matrix_b, expected);
}

#[test]
fn test_multiplying_a_matrix_by_the_identity() {
    let matrix = Matrix4::from_rows([
        [0.0, 1.0, 2.0, 4.0],
        [1.0, 2.0, 4.0, 8.0],
        [2.0, 4.0, 8.0, 16.0],
        [4.0, 8.0, 16.0, 32.0],
    ]);
    assert_eq!(matrix.clone() * IDENTITY_MATRIX4, matrix);
}

impl ops::Mul<Tuple> for Matrix4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        Tuple::new(
            self.rows[0][0] as f32 * rhs.x
                + self.rows[0][1] as f32 * rhs.y
                + self.rows[0][2] as f32 * rhs.z
                + self.rows[0][3] as f32 * rhs.w,
            self.rows[1][0] as f32 * rhs.x
                + self.rows[1][1] as f32 * rhs.y
                + self.rows[1][2] as f32 * rhs.z
                + self.rows[1][3] as f32 * rhs.w,
            self.rows[2][0] as f32 * rhs.x
                + self.rows[2][1] as f32 * rhs.y
                + self.rows[2][2] as f32 * rhs.z
                + self.rows[2][3] as f32 * rhs.w,
            self.rows[3][0] as f32 * rhs.x
                + self.rows[3][1] as f32 * rhs.y
                + self.rows[3][2] as f32 * rhs.z
                + self.rows[3][3] as f32 * rhs.w,
        )
    }
}

#[test]
fn test_matrix_multiplied_by_a_tuple() {
    let matrix = Matrix4::from_rows([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
    assert_eq!(matrix * tuple, Tuple::new(18.0, 24.0, 33.0, 1.0));
}

#[test]
fn test_multiplying_identity_by_a_tuple() {
    let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(IDENTITY_MATRIX4 * tuple, tuple);
}

impl Matrix4 {
    pub fn from_rows(rows: [[f32; 4]; 4]) -> Self {
        Matrix4 { rows }
    }

    pub fn transpose(&self) -> Self {
        Matrix4::from_rows([
            [
                self.rows[0][0],
                self.rows[1][0],
                self.rows[2][0],
                self.rows[3][0],
            ],
            [
                self.rows[0][1],
                self.rows[1][1],
                self.rows[2][1],
                self.rows[3][1],
            ],
            [
                self.rows[0][2],
                self.rows[1][2],
                self.rows[2][2],
                self.rows[3][2],
            ],
            [
                self.rows[0][3],
                self.rows[1][3],
                self.rows[2][3],
                self.rows[3][3],
            ],
        ])
    }
}

pub const IDENTITY_MATRIX4: Matrix4 = Matrix4 {
    rows: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

#[test]
fn test_constructing_and_inspecting_a_4x4_matrix() {
    let matrix = Matrix4::from_rows([
        [1.0, 2.0, 3.0, 4.0],
        [5.5, 6.5, 7.5, 8.5],
        [9.0, 10.0, 11.0, 12.0],
        [13.5, 14.5, 15.5, 16.5],
    ]);
    assert_eq!(matrix.rows[0][0], 1.0);
    assert_eq!(matrix.rows[0][3], 4.0);
    assert_eq!(matrix.rows[1][0], 5.5);
    assert_eq!(matrix.rows[1][2], 7.5);
    assert_eq!(matrix.rows[2][2], 11.0);
    assert_eq!(matrix.rows[3][0], 13.5);
    assert_eq!(matrix.rows[3][2], 15.5);
}

#[test]
fn test_matrix_equality_with_identical_matrices() {
    let matrix_a = Matrix4::from_rows([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 3.0, 4.0, 5.0],
        [3.0, 4.0, 5.0, 6.0],
        [4.0, 5.0, 6.0, 7.0],
    ]);
    let matrix_b = Matrix4::from_rows([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 3.0, 4.0, 5.0],
        [3.0, 4.0, 5.0, 6.0],
        [4.0, 5.0, 6.0, 7.0],
    ]);
    assert_eq!(matrix_a, matrix_b);
}

#[test]
fn test_matrix_equality_with_different_matrices() {
    let matrix_a = Matrix4::from_rows([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 3.0, 4.0, 5.0],
        [3.0, 4.0, 5.0, 6.0],
        [4.0, 5.0, 6.0, 7.0],
    ]);
    let matrix_b = Matrix4::from_rows([
        [0.0, 2.0, 3.0, 4.0],
        [2.0, 3.0, 4.0, 5.0],
        [3.0, 4.0, 5.0, 6.0],
        [4.0, 5.0, 6.0, 7.0],
    ]);
    assert!(matrix_a != matrix_b);
}

#[test]
fn test_transposing_a_matrix() {
    let matrix = Matrix4::from_rows([
        [0.0, 9.0, 3.0, 0.0],
        [9.0, 8.0, 0.0, 8.0],
        [1.0, 8.0, 5.0, 3.0],
        [0.0, 0.0, 5.0, 8.0],
    ]);
    let expected = Matrix4::from_rows([
        [0.0, 9.0, 1.0, 0.0],
        [9.0, 8.0, 8.0, 0.0],
        [3.0, 0.0, 5.0, 5.0],
        [0.0, 8.0, 3.0, 8.0],
    ]);
    assert_eq!(matrix.transpose(), expected);
}

#[test]
fn test_transposing_the_identity_matrix() {
    assert_eq!(IDENTITY_MATRIX4.transpose(), IDENTITY_MATRIX4);
}
