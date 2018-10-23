use super::EPSILON;
use std::default::Default;
use std::fmt;
use std::ops;
use tuples::Tuple;

#[derive(Copy, Clone, Debug)]
pub struct Matrix2 {
    pub rows: [[f32; 2]; 2],
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Matrix2) -> bool {
        for row in 0..2 {
            for col in 0..2 {
                if (self.rows[row][col] - other.rows[row][col]).abs() > EPSILON
                {
                    return false;
                }
            }
        }
        true
    }
}

impl Matrix2 {
    pub fn from_rows(rows: [[f32; 2]; 2]) -> Self {
        Matrix2 { rows }
    }

    pub fn determinant(&self) -> f32 {
        self.rows[0][0] * self.rows[1][1] - self.rows[0][1] * self.rows[1][0]
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

#[test]
fn test_calculating_the_determinant_of_a_2x2_matrix() {
    let matrix = Matrix2::from_rows([[1.0, 5.0], [-3.0, 2.0]]);
    assert_eq!(matrix.determinant(), 17.0);
}

#[derive(Copy, Clone, Debug)]
pub struct Matrix3 {
    pub rows: [[f32; 3]; 3],
}

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Matrix3) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if (self.rows[row][col] - other.rows[row][col]).abs() > EPSILON
                {
                    return false;
                }
            }
        }
        true
    }
}

impl Matrix3 {
    pub fn from_rows(rows: [[f32; 3]; 3]) -> Self {
        Matrix3 { rows }
    }

    pub fn submatrix(&self, del_row: usize, del_col: usize) -> Matrix2 {
        let mut values = Vec::with_capacity(2 * 2);
        for (rowi, row) in self.rows.iter().enumerate() {
            for (coli, value) in row.iter().enumerate() {
                if rowi != del_row && coli != del_col {
                    values.push(value.clone());
                }
            }
        }
        Matrix2::from_rows([[values[0], values[1]], [values[2], values[3]]])
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let mut val = self.minor(row, col);
        if (row + col) % 2 != 0 {
            val = -val
        }
        val
    }

    pub fn determinant(&self) -> f32 {
        self.rows[0][0] * self.cofactor(0, 0)
            + self.rows[0][1] * self.cofactor(0, 1)
            + self.rows[0][2] * self.cofactor(0, 2)
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

#[test]
fn test_a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
    let matrix = Matrix3::from_rows([
        [1.0, 5.0, 0.0],
        [-3.0, 2.0, 7.0],
        [0.0, 6.0, -3.0],
    ]);
    let expected = Matrix2::from_rows([[-3.0, 2.0], [0.0, 6.0]]);
    assert_eq!(matrix.submatrix(0, 2), expected);
}

#[test]
fn test_calculating_a_minor_or_a_3x3_matrix() {
    let matrix = Matrix3::from_rows([
        [3.0, 5.0, 0.0],
        [2.0, -1.0, -7.0],
        [6.0, -1.0, 5.0],
    ]);
    let submatrix = matrix.submatrix(1, 0);
    assert_eq!(submatrix.determinant(), 25.0);
    assert_eq!(matrix.minor(1, 0), 25.0);
}

#[test]
fn test_calculating_a_cofactor_of_a_3x3_matrix() {
    let matrix = Matrix3::from_rows([
        [3.0, 5.0, 0.0],
        [2.0, -1.0, -7.0],
        [6.0, -1.0, 5.0],
    ]);
    assert_eq!(matrix.minor(0, 0), -12.0);
    assert_eq!(matrix.cofactor(0, 0), -12.0);
    assert_eq!(matrix.minor(1, 0), 25.0);
    assert_eq!(matrix.cofactor(1, 0), -25.0);
}

#[test]
fn test_calculating_the_determinant_of_a_3x3_matrix() {
    let matrix = Matrix3::from_rows([
        [1.0, 2.0, 6.0],
        [-5.0, 8.0, -4.0],
        [2.0, 6.0, 4.0],
    ]);
    assert_eq!(matrix.cofactor(0, 0), 56.0);
    assert_eq!(matrix.cofactor(0, 1), 12.0);
    assert_eq!(matrix.cofactor(0, 2), -46.0);
    assert_eq!(matrix.determinant(), -196.0);
}

#[derive(Copy, Clone)]
pub struct Matrix4 {
    pub rows: [[f32; 4]; 4],
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Matrix4) -> bool {
        for row in 0..4 {
            for col in 0..4 {
                if (self.rows[row][col] - other.rows[row][col]).abs() > EPSILON
                {
                    return false;
                }
            }
        }
        true
    }
}

impl fmt::Debug for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..4 {
            for col in 0..4 {
                write!(f, " | {:3.5}", self.rows[row][col])?;
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

impl ops::Div<f32> for Matrix4 {
    type Output = Matrix4;
    fn div(self, other: f32) -> Matrix4 {
        Matrix4::from_rows([
            [
                self.rows[0][0] / other,
                self.rows[0][1] / other,
                self.rows[0][2] / other,
                self.rows[0][3] / other,
            ],
            [
                self.rows[1][0] / other,
                self.rows[1][1] / other,
                self.rows[1][2] / other,
                self.rows[1][3] / other,
            ],
            [
                self.rows[2][0] / other,
                self.rows[2][1] / other,
                self.rows[2][2] / other,
                self.rows[2][3] / other,
            ],
            [
                self.rows[3][0] / other,
                self.rows[3][1] / other,
                self.rows[3][2] / other,
                self.rows[3][3] / other,
            ],
        ])
    }
}

impl Matrix4 {
    pub fn from_rows(rows: [[f32; 4]; 4]) -> Self {
        Matrix4 { rows }
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = IDENTITY_MATRIX4;
        matrix.rows[0][3] = x;
        matrix.rows[1][3] = y;
        matrix.rows[2][3] = z;
        matrix
    }

    pub fn scaling(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = IDENTITY_MATRIX4;
        matrix.rows[0][0] = x;
        matrix.rows[1][1] = y;
        matrix.rows[2][2] = z;
        matrix
    }

    pub fn rotation_x(angle: f32) -> Self {
        let mut matrix = IDENTITY_MATRIX4;
        matrix.rows[1][1] = angle.cos();
        matrix.rows[1][2] = -angle.sin();
        matrix.rows[2][1] = angle.sin();
        matrix.rows[2][2] = angle.cos();
        matrix
    }

    pub fn rotation_y(angle: f32) -> Self {
        let mut matrix = IDENTITY_MATRIX4;
        matrix.rows[0][0] = angle.cos();
        matrix.rows[0][2] = angle.sin();
        matrix.rows[2][0] = -angle.sin();
        matrix.rows[2][2] = angle.cos();
        matrix
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

    pub fn submatrix(&self, del_row: usize, del_col: usize) -> Matrix3 {
        let mut values = Vec::with_capacity(3 * 3);
        for (rowi, row) in self.rows.iter().enumerate() {
            for (coli, value) in row.iter().enumerate() {
                if rowi != del_row && coli != del_col {
                    values.push(value.clone());
                }
            }
        }
        Matrix3::from_rows([
            [values[0], values[1], values[2]],
            [values[3], values[4], values[5]],
            [values[6], values[7], values[8]],
        ])
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let mut val = self.minor(row, col);
        if (row + col) % 2 != 0 {
            val = -val
        }
        val
    }

    pub fn determinant(&self) -> f32 {
        self.rows[0][0] * self.cofactor(0, 0)
            + self.rows[0][1] * self.cofactor(0, 1)
            + self.rows[0][2] * self.cofactor(0, 2)
            + self.rows[0][3] * self.cofactor(0, 3)
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Self {
        let mut cofactors = Vec::with_capacity(4 * 4);
        for row in 0..4 {
            for col in 0..4 {
                cofactors.push(self.cofactor(row, col));
            }
        }
        let cofactor_matrix = Matrix4::from_rows([
            [cofactors[0], cofactors[1], cofactors[2], cofactors[3]],
            [cofactors[4], cofactors[5], cofactors[6], cofactors[7]],
            [cofactors[8], cofactors[9], cofactors[10], cofactors[11]],
            [cofactors[12], cofactors[13], cofactors[14], cofactors[15]],
        ]);
        let transposed = cofactor_matrix.transpose();
        transposed / self.determinant()
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

#[test]
fn test_a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
    let matrix = Matrix4::from_rows([
        [-6.0, 1.0, 1.0, 6.0],
        [-8.0, 5.0, 8.0, 6.0],
        [-1.0, 0.0, 8.0, 2.0],
        [-7.0, 1.0, -1.0, 1.0],
    ]);
    let expected = Matrix3::from_rows([
        [-6.0, 1.0, 6.0],
        [-8.0, 8.0, 6.0],
        [-7.0, -1.0, 1.0],
    ]);
    assert_eq!(matrix.submatrix(2, 1), expected);
}

#[test]
fn test_calculating_the_determinant_of_a_4x4_matrix() {
    let matrix = Matrix4::from_rows([
        [-2.0, -8.0, 3.0, 5.0],
        [-3.0, 1.0, 7.0, 3.0],
        [1.0, 2.0, -9.0, 6.0],
        [-6.0, 7.0, 7.0, -9.0],
    ]);
    assert_eq!(matrix.cofactor(0, 0), 690.0);
    assert_eq!(matrix.cofactor(0, 1), 447.0);
    assert_eq!(matrix.cofactor(0, 2), 210.0);
    assert_eq!(matrix.cofactor(0, 3), 51.0);
    assert_eq!(matrix.determinant(), -4071.0);
}

#[test]
fn test_testing_an_invertible_matrix_for_invertibility() {
    let matrix = Matrix4::from_rows([
        [6.0, 4.0, 4.0, 4.0],
        [5.0, 5.0, 7.0, 6.0],
        [4.0, -9.0, 3.0, -7.0],
        [9.0, 1.0, 7.0, -6.0],
    ]);
    assert_eq!(matrix.determinant(), -2120.0);
    assert!(matrix.is_invertible());
}

#[test]
fn test_testing_a_non_invertible_matrix_for_invertibility() {
    let matrix = Matrix4::from_rows([
        [-4.0, 2.0, -2.0, -3.0],
        [9.0, 6.0, 2.0, 6.0],
        [0.0, -5.0, 1.0, -5.0],
        [0.0, 0.0, 0.0, 0.0],
    ]);
    assert_eq!(matrix.determinant(), 0.0);
    assert!(!matrix.is_invertible());
}

#[test]
fn test_calculating_the_inverse_of_a_matrix() {
    let matrix = Matrix4::from_rows([
        [-5.0, 2.0, 6.0, -8.0],
        [1.0, -5.0, 1.0, 8.0],
        [7.0, 7.0, -6.0, -7.0],
        [1.0, -3.0, 7.0, 4.0],
    ]);
    let inverse = matrix.inverse();
    assert_eq!(matrix.determinant(), 532.0);
    assert_eq!(matrix.cofactor(2, 3), -160.0);
    assert_eq!(inverse.rows[3][2], -160.0 / 532.0);
    assert_eq!(matrix.cofactor(3, 2), 105.0);
    assert_eq!(inverse.rows[2][3], 105.0 / 532.0);
    let expected = Matrix4::from_rows([
        [0.21805, 0.45113, 0.24060, -0.04511],
        [-0.80827, -1.45677, -0.44361, 0.52068],
        [-0.07895, -0.22368, -0.05263, 0.19737],
        [-0.52256, -0.81391, -0.30075, 0.30639],
    ]);
    assert_eq!(inverse, expected);
}

#[test]
fn test_calculating_the_inverse_of_another_matrix() {
    let matrix = Matrix4::from_rows([
        [8.0, -5.0, 9.0, 2.0],
        [7.0, 5.0, 6.0, 1.0],
        [-6.0, 0.0, 9.0, 6.0],
        [-3.0, 0.0, -9.0, -4.0],
    ]);
    let expected = Matrix4::from_rows([
        [-0.15385, -0.15385, -0.28205, -0.53846],
        [-0.07692, 0.12308, 0.02564, 0.03077],
        [0.35897, 0.35897, 0.43590, 0.92308],
        [-0.69231, -0.69231, -0.76923, -1.92308],
    ]);
    assert_eq!(matrix.inverse(), expected);
}

#[test]
fn test_calculating_the_inverse_of_a_third_matrix() {
    let matrix = Matrix4::from_rows([
        [9.0, 3.0, 0.0, 9.0],
        [-5.0, -2.0, -6.0, -3.0],
        [-4.0, 9.0, 6.0, 4.0],
        [-7.0, 6.0, 6.0, 2.0],
    ]);
    let expected = Matrix4::from_rows([
        [-0.04074, -0.07778, 0.14444, -0.22222],
        [-0.07778, 0.03333, 0.36667, -0.33333],
        [-0.02901, -0.14630, -0.10926, 0.12963],
        [0.17778, 0.06667, -0.26667, 0.33333],
    ]);
    assert_eq!(matrix.inverse(), expected);
}

#[test]
fn test_multiplying_a_product_by_its_inverse() {
    let matrix_a = Matrix4::from_rows([
        [3.0, -9.0, 7.0, 3.0],
        [3.0, -8.0, 2.0, -9.0],
        [-4.0, 4.0, 4.0, 1.0],
        [-6.0, 5.0, -1.0, 1.0],
    ]);
    let matrix_b = Matrix4::from_rows([
        [8.0, 2.0, 2.0, 2.0],
        [3.0, -1.0, 7.0, 0.0],
        [7.0, 0.0, 5.0, 4.0],
        [6.0, -2.0, 0.0, 5.0],
    ]);
    let c = matrix_a * matrix_b;
    assert_eq!(c * matrix_b.inverse(), matrix_a);
}

#[test]
fn test_multiplying_by_a_translation_matrix() {
    let transform = Matrix4::translation(5.0, -3.0, 2.0);
    let p = Tuple::point(-3.0, 4.0, 5.0);
    assert_eq!(transform * p, Tuple::point(2.0, 1.0, 7.0));
}

#[test]
fn test_multiplying_by_the_inverse_of_a_translation_matrix() {
    let transform = Matrix4::translation(5.0, -3.0, 2.0);
    let inv = transform.inverse();
    let p = Tuple::point(-3.0, 4.0, 5.0);
    assert_eq!(inv * p, Tuple::point(-8.0, 7.0, 3.0));
}

#[test]
fn test_translation_does_not_affect_vectors() {
    let transform = Matrix4::translation(5.0, -3.0, 2.0);
    let v = Tuple::vector(-3.0, 4.0, 5.0);
    assert_eq!(transform * v, v);
}

#[test]
fn test_scaling_matrix_applied_to_a_point() {
    let transform = Matrix4::scaling(2.0, 3.0, 4.0);
    let p = Tuple::point(-4.0, 6.0, 8.0);
    assert_eq!(transform * p, Tuple::point(-8.0, 18.0, 32.0));
}

#[test]
fn test_a_scaling_matrix_applied_to_a_vector() {
    let transform = Matrix4::scaling(2.0, 3.0, 4.0);
    let v = Tuple::vector(-4.0, 6.0, 8.0);
    assert_eq!(transform * v, Tuple::vector(-8.0, 18.0, 32.0));
}

#[test]
fn test_multiplying_by_the_inverse_of_a_scaling_matrix() {
    let transform = Matrix4::scaling(2.0, 3.0, 4.0);
    let inv = transform.inverse();
    let v = Tuple::vector(-4.0, 6.0, 8.0);
    assert_eq!(inv * v, Tuple::vector(-2.0, 2.0, 2.0));
}

#[test]
fn test_reflection_is_scaling_by_a_negative_value() {
    let transform = Matrix4::scaling(-1.0, 1.0, 1.0);
    let p = Tuple::point(2.0, 3.0, 4.0);
    assert_eq!(transform * p, Tuple::point(-2.0, 3.0, 4.0));
}

#[test]
fn test_rotating_a_point_around_the_x_axis() {
    use std::f32::consts::PI;

    let p = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter = Matrix4::rotation_x(PI / 4.0);
    let full_quarter = Matrix4::rotation_x(PI / 2.0);
    assert_eq!(
        half_quarter * p,
        Tuple::point(0.0, 2f32.sqrt() / 2.0, 2f32.sqrt() / 2.0)
    );
    assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
}

#[test]
fn test_the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
    use std::f32::consts::PI;

    let v = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter = Matrix4::rotation_x(PI / 4.0);
    let inv = half_quarter.inverse();
    assert_eq!(
        inv * v,
        Tuple::point(0.0, 2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0)
    );
}

#[test]
fn test_rotating_a_point_around_the_y_axis() {
    use std::f32::consts::PI;

    let p = Tuple::point(0.0, 0.0, 1.0);
    let half_quarter = Matrix4::rotation_y(PI / 4.0);
    let full_quarter = Matrix4::rotation_y(PI / 2.0);
    assert_eq!(
        half_quarter * p,
        Tuple::point(2f32.sqrt() / 2.0, 0.0, 2f32.sqrt() / 2.0)
    );
    assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
}
