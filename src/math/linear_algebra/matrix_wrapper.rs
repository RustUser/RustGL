use std::ops::{Add, Index, IndexMut, Mul, Sub};
use vecmath::{mat4_transposed, Matrix4, Vector3};
use crate::math::linear_algebra::IDENTITY_MAT4;
use crate::math::linear_algebra::types::Vec3;
use crate::{translation, v3};
use crate::math::linear_algebra::vector_wrapper::Vec3Wrapper;

#[macro_export]
macro_rules! mat4 {
    ($matrix:expr) => {
        MatrixWrapper($matrix)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MatrixWrapper(pub Matrix4<f32>);

pub struct Float(f32);

impl Mul<Float> for [f32; 4] {
    type Output = [f32; 4];

    fn mul(self, rhs: Float) -> Self::Output {
        [self[0] * rhs.0, self[1] * rhs.0, self[2] * rhs.0, self[3] * rhs.0]
    }
}

impl MatrixWrapper {
    pub fn new(matrix: Matrix4<f32>) -> MatrixWrapper {
        Self(matrix)
    }
    pub fn to_owned(self) -> Matrix4<f32> {
        self.0
    }

    pub fn translated(&self, t: Vector3<f32>) -> MatrixWrapper {
        MatrixWrapper(translation(t)) + *self
    }

    pub fn scale(&self, scale: Vector3<f32>) -> MatrixWrapper {
        let s = self.0.clone();
        let c0 = s[0] * Float(scale[0]);
        let c1 = s[1] * Float(scale[1]);
        let c2 = s[2] * Float(scale[2]);
        let c3 = s[3];
        Self([
            c0,
            c1,
            c2,
            c3
        ])
    }

    pub fn translation(t: Vector3<f32>) -> MatrixWrapper {
        MatrixWrapper(translation(t))
    }

    pub fn scale_matrix(scale: Vector3<f32>) -> MatrixWrapper {
        MatrixWrapper(IDENTITY_MAT4)
            .with_value(0, 0, scale[0])
            .with_value(1, 1, scale[1])
            .with_value(2, 2, scale[2])
    }

    pub fn with_value(mut self, x: usize, y: usize, value: f32) -> Self {
        self.0[x][y] = value;
        self
    }
    pub fn transposed(self) -> MatrixWrapper {
        Self(mat4_transposed(self.0))
    }

    pub fn look_at(
        eye: Vec3,
        center: Vec3,
        up: Vec3,
    ) -> MatrixWrapper {
        let zero = 0f32;
        let one = 1f32;
        let f = (v3!(center) - v3!(eye)).normalized();
        let s = f.cross(&v3!(up)).normalized();
        let u = s.cross(&f);

        let eye = v3!(eye);

        let mat = [
            [s.x(), u.x(), - f.x(), zero],
            [s.y(), u.y(), - f.y(), zero],
            [s.z(), u.z(), -f.z(), zero],
            [-s.dot(&eye), -u.dot(&eye), f.dot(&eye), one]
        ];
        Self(mat)
    }
}

impl Index<usize> for MatrixWrapper {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for MatrixWrapper {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<Matrix4<f32>> for MatrixWrapper {
    fn from(matrix: Matrix4<f32>) -> Self {
        MatrixWrapper::new(matrix)
    }
}

impl Add<MatrixWrapper> for MatrixWrapper {
    type Output = MatrixWrapper;

    fn add(self, rhs: MatrixWrapper) -> Self::Output {
        let mut result = [[0f32; 4]; 4];
        for i in 0..result.len() {
            for j in 0..result[i].len() {
                result[i][j] = self[i][j] + rhs[i][j];
            }
        }
        MatrixWrapper::from(result)
    }
}

impl Sub<MatrixWrapper> for MatrixWrapper {
    type Output = MatrixWrapper;

    fn sub(self, rhs: MatrixWrapper) -> Self::Output {
        let mut result = [[0f32; 4]; 4];
        for i in 0..result.len() {
            for j in 0..result[i].len() {
                result[i][j] = self[i][j] - rhs[i][j];
            }
        }
        MatrixWrapper::from(result)
    }
}

impl Mul<MatrixWrapper> for MatrixWrapper {
    type Output = MatrixWrapper;

    fn mul(self, rhs: MatrixWrapper) -> Self::Output {
        let multiply_matrices_cell = |first: &MatrixWrapper, second: &MatrixWrapper, row: usize, col: usize| {
            let mut cell = 0f32;
            for i in 0..second[0].len() {
                cell += first[row][i] * second[i][col];
            }
            cell
        };
        let mut result = [[0f32; 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                result[row][col] = multiply_matrices_cell(&self, &rhs, row, col);
            }
        }

        MatrixWrapper::new(result)
    }
}