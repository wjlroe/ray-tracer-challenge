#[derive(Debug, PartialEq)]
pub struct Matrix2<T> {
    pub rows: [[T; 2]; 2],
}

impl<T> Matrix2<T> {
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
pub struct Matrix3<T> {
    pub rows: [[T; 3]; 3],
}

impl<T> Matrix3<T> {
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

#[derive(Debug, PartialEq)]
pub struct Matrix4<T> {
    pub rows: [[T; 4]; 4],
}

pub type Matrix4f32 = Matrix4<f32>;

impl<T> Matrix4<T> {
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
