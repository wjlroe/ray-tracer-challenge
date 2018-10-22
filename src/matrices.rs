use num::Num;
use std::default::Default;
use std::fmt;
use std::ops;
use tuples::{Tuple, T as TupleT};

#[derive(Debug, PartialEq)]
pub struct Matrix2<Num> {
    pub rows: [[Num; 2]; 2],
}

impl<T: Num> Matrix2<T> {
    pub fn from_rows(rows: [[T; 2]; 2]) -> Self {
        Matrix2 { rows }
    }
}

#[test]
fn test_a_2x2_matrix_should_be_representable() {
    let matrix = Matrix2::from_rows([[-3, 5], [1, -2]]);
    assert_eq!(matrix.rows[0][0], -3);
    assert_eq!(matrix.rows[0][1], 5);
    assert_eq!(matrix.rows[1][0], 1);
    assert_eq!(matrix.rows[1][1], -2);
}

#[derive(Debug, PartialEq)]
pub struct Matrix3<Num> {
    pub rows: [[Num; 3]; 3],
}

impl<T: Num> Matrix3<T> {
    pub fn from_rows(rows: [[T; 3]; 3]) -> Self {
        Matrix3 { rows }
    }
}

#[test]
fn test_a_3x3_matrix_should_be_representable() {
    let matrix = Matrix3::from_rows([[-3, 5, 0], [1, -2, -7], [0, 1, 1]]);
    assert_eq!(matrix.rows[0][0], -3);
    assert_eq!(matrix.rows[1][1], -2);
    assert_eq!(matrix.rows[2][2], 1);
}

#[derive(PartialEq)]
pub struct Matrix4<Num> {
    pub rows: [[Num; 4]; 4],
}

pub type Matrix4f32 = Matrix4<f32>;

impl<T: Num> Matrix4<T> {
    pub fn from_rows(rows: [[T; 4]; 4]) -> Self {
        Matrix4 { rows }
    }
}

#[test]
fn test_constructing_and_inspecting_a_4x4_matrix() {
    let matrix = Matrix4f32::from_rows([
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
        [1, 2, 3, 4],
        [2, 3, 4, 5],
        [3, 4, 5, 6],
        [4, 5, 6, 7],
    ]);
    let matrix_b = Matrix4::from_rows([
        [1, 2, 3, 4],
        [2, 3, 4, 5],
        [3, 4, 5, 6],
        [4, 5, 6, 7],
    ]);
    assert_eq!(matrix_a, matrix_b);
}

#[test]
fn test_matrix_equality_with_different_matrices() {
    let matrix_a = Matrix4::from_rows([
        [1, 2, 3, 4],
        [2, 3, 4, 5],
        [3, 4, 5, 6],
        [4, 5, 6, 7],
    ]);
    let matrix_b = Matrix4::from_rows([
        [0, 2, 3, 4],
        [2, 3, 4, 5],
        [3, 4, 5, 6],
        [4, 5, 6, 7],
    ]);
    assert!(matrix_a != matrix_b);
}

impl<T: Num> fmt::Debug for Matrix4<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..4 {
            for col in 0..4 {
                write!(f, " | {:3}", self.rows[row][col])?;
            }
            write!(f, " |")?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T: Num> ops::Mul<Matrix4<T>> for Matrix4<T> {
    type Output = Self;

    fn mul(self, other: Matrix4<T>) -> Self {
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
        [1, 2, 3, 4],
        [2, 3, 4, 5],
        [3, 4, 5, 6],
        [4, 5, 6, 7],
    ]);
    let matrix_b = Matrix4::from_rows([
        [0, 1, 2, 4],
        [1, 2, 4, 8],
        [2, 4, 8, 16],
        [4, 8, 16, 32],
    ]);
    let expected = Matrix4::from_rows([
        [24, 49, 98, 196],
        [31, 64, 128, 256],
        [38, 79, 158, 316],
        [45, 94, 188, 376],
    ]);
    assert_eq!(matrix_a * matrix_b, expected);
}

impl<TupleT, M: Num> ops::Mul<Tuple<TupleT>> for Matrix4<M> {
    type Output = Tuple<TupleT>;

    fn mul(self, rhs: Tuple<TupleT>) -> Tuple<TupleT> {
        Tuple::new(
            self.rows[0][0] as f32 * rhs.x
                + self.rows[0][1] as f32 * rhs.y
                + self.rows[0][2] as f32 * rhs.z
                + self.rows[0][3] as f32 * rhs.w,
            0.0,
            0.0,
            0.0,
        )
    }
}

#[test]
fn test_matrix_multiplied_by_a_tuple() {
    let matrix = Matrix4::from_rows([
        [1, 2, 3, 4],
        [2, 4, 4, 2],
        [8, 6, 4, 1],
        [0, 0, 0, 1],
    ]);
    let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
    assert_eq!(matrix * tuple, Tuple::new(18.0, 24.0, 33.0, 1.0));
}
