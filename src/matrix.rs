use crate::Tuple;
use num_traits::{Float, Num};
use std::ops::{Add, AddAssign, Div, Index, Mul, Sub};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Matrix<T, const Y: usize, const X: usize> {
    pub mat: [[T; X]; Y],
}

impl<T: Num + AddAssign + Default + Copy + Float, const Y: usize, const X: usize> Matrix<T, Y, X> {
    pub fn new(mat: [[T; X]; Y]) -> Self {
        Matrix { mat }
    }

    pub fn transpose(&self) -> Matrix<T, X, Y> {
        let t = T::default();
        let mut transposed = [[t; Y]; X];

        for y in 0..Y {
            for x in 0..X {
                transposed[y][x] = self[x][y];
            }
        }

        Matrix { mat: transposed }
    }

    pub fn norm(&self) -> T {
        let mut euclidean_norm = T::default();
        for y in 0..Y {
            for x in 0..X {
                euclidean_norm += self[y][x] * self[y][x];
            }
        }

        euclidean_norm.sqrt()
    }
}

impl<T, const Y: usize, const X: usize> Index<usize> for Matrix<T, Y, X> {
    type Output = [T; X];

    fn index(&self, index: usize) -> &Self::Output {
        &self.mat[index]
    }
}

impl<T: Num + Copy + Default, const Y: usize, const X: usize> Add<Matrix<T, Y, X>>
    for Matrix<T, Y, X>
{
    type Output = Matrix<T, Y, X>;

    fn add(self, rhs: Matrix<T, Y, X>) -> Self::Output {
        let t = T::default();
        let mut result = [[t; X]; Y];
        for y in 0..Y {
            for x in 0..X {
                result[y][x] = self[y][x] + rhs[y][x];
            }
        }
        Matrix { mat: result }
    }
}

impl<T: Num + Copy + Default, const Y: usize, const X: usize> Sub<Matrix<T, Y, X>>
    for Matrix<T, Y, X>
{
    type Output = Matrix<T, Y, X>;

    fn sub(self, rhs: Matrix<T, Y, X>) -> Self::Output {
        let t = T::default();
        let mut result = [[t; X]; Y];
        for y in 0..Y {
            for x in 0..X {
                result[y][x] = self[y][x] - rhs[y][x];
            }
        }
        Matrix { mat: result }
    }
}

impl<T: Num + AddAssign + Copy + Default, const Y: usize, const X: usize, const M: usize>
    Mul<Matrix<T, M, X>> for Matrix<T, Y, M>
{
    type Output = Matrix<T, Y, X>;

    fn mul(self, rhs: Matrix<T, M, X>) -> Self::Output {
        let t = T::default();
        let mut result = [[t; X]; Y];
        for y in 0..Y {
            for x in 0..X {
                for m in 0..M {
                    result[y][x] += self[y][m] * rhs[m][x]
                }
            }
        }
        Matrix { mat: result }
    }
}

impl<T: Num + AddAssign + Copy + Default, const Y: usize, const X: usize> Mul<T>
    for Matrix<T, Y, X>
{
    type Output = Matrix<T, Y, X>;
    fn mul(self, rhs: T) -> Self::Output {
        let t = T::default();
        let mut product = [[t; X]; Y];
        for y in 0..Y {
            for x in 0..X {
                product[y][x] = self[y][x] * rhs;
            }
        }
        Matrix { mat: product }
    }
}

impl<T: Num + AddAssign + Copy + Default, const Y: usize, const X: usize> Div<T>
    for Matrix<T, Y, X>
{
    type Output = Matrix<T, Y, X>;
    fn div(self, rhs: T) -> Self::Output {
        let t = T::default();
        let mut product = [[t; X]; Y];
        for y in 0..Y {
            for x in 0..X {
                product[y][x] = self[y][x] / rhs;
            }
        }
        Matrix { mat: product }
    }
}

impl Mul<Tuple> for Matrix<f32, 4, 4> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let x = self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z + self[0][3] * rhs.w;
        let y = self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z + self[1][3] * rhs.w;
        let z = self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z + self[2][3] * rhs.w;
        let w = self[3][0] * rhs.x + self[3][1] * rhs.y + self[3][2] * rhs.z + self[3][3] * rhs.w;

        Tuple::new(x, y, z, w)
    }
}

impl Matrix<f32, 4, 4> {
    pub fn submatrix(self, r: usize, c: usize) -> Matrix<f32, 3, 3> {
        let mut sub = [[0.0; 3]; 3];

        for y in 0..3 {
            for x in 0..3 {
                let skipped_col = y >= r;
                let skipped_row = x >= c;

                sub[y][x] = match (skipped_col, skipped_row) {
                    (false, false) => self[y][x],
                    (true, false) => self[y + 1][x],
                    (false, true) => self[y][x + 1],
                    (true, true) => self[y + 1][x + 1],
                };
            }
        }
        Matrix { mat: sub }
    }

    pub fn minor(self, r: usize, c: usize) -> f32 {
        self.submatrix(r, c).determinant()
    }

    pub fn cofactor(self, r: usize, c: usize) -> f32 {
        let pow = (r + c) % 2;
        if pow == 0 {
            self.minor(r, c)
        } else {
            -self.minor(r, c)
        }
    }

    pub fn determinant(self) -> f32 {
        let mut det = 0.;
        for r in 0..4 {
            det += self[r][0] * self.cofactor(r, 0);
        }
        det
    }

    pub fn invertible(self) -> bool {
        self.determinant() != 0.
    }

    pub fn inverse(self) -> Result<Self, &'static str> {
        if !self.invertible() {
            return Err("Cannot be inverted. Determinant 0");
        }

        let det = self.determinant();
        let mut inverse = [[0.; 4]; 4];

        for r in 0..4 {
            for c in 0..4 {
                let cof = self.cofactor(r, c);
                inverse[c][r] = cof / det;
            }
        }
        Ok(Self::new(inverse))
    }
}

impl Matrix<f32, 3, 3> {
    pub fn submatrix(self, r: usize, c: usize) -> Matrix<f32, 2, 2> {
        let mut sub = [[0.0; 2]; 2];

        for y in 0..2 {
            for x in 0..2 {
                let skipped_col = y >= r;
                let skipped_row = x >= c;

                sub[y][x] = match (skipped_col, skipped_row) {
                    (false, false) => self[y][x],
                    (true, false) => self[y + 1][x],
                    (false, true) => self[y][x + 1],
                    (true, true) => self[y + 1][x + 1],
                };
            }
        }
        Matrix { mat: sub }
    }

    pub fn minor(self, r: usize, c: usize) -> f32 {
        self.submatrix(r, c).determinant()
    }

    pub fn cofactor(self, r: usize, c: usize) -> f32 {
        let pow = (r + c) % 2;
        if pow == 0 {
            self.minor(r, c)
        } else {
            -self.minor(r, c)
        }
    }

    pub fn determinant(self) -> f32 {
        let mut det = 0.;
        for r in 0..3 {
            det += self[r][0] * self.cofactor(r, 0);
        }
        det
    }

    pub fn invertible(self) -> bool {
        self.determinant() != 0.
    }
}

impl Matrix<f32, 2, 2> {
    pub fn determinant(self) -> f32 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    pub fn invertible(self) -> bool {
        self.determinant() != 0.
    }
}
