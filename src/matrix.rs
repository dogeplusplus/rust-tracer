use std::ops::{Index, Mul};
use crate::Tuple;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
    pub mat: Vec<Vec<f32>>,
    pub rows: usize,
    pub columns: usize,
}

impl Matrix {
    pub fn new(mat: Vec<Vec<f32>>) -> Self {
        let rows = mat.len();
        let columns = mat[0].len();
        Matrix { mat, rows, columns }
    }

    pub fn transpose(&self) -> Self {
        let rows = self.columns;
        let columns = self.rows;

        let mut mat = Vec::new();
        for r in 0..rows {
            let mut transposed_row = Vec::new(); 
            for c in 0..columns {
                transposed_row.push(self[c][r]);
            }
            mat.push(transposed_row);
        }

        Matrix { mat, rows, columns }
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.mat[index]
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.columns, rhs.rows);

        let mut product = Vec::new();
        let mid = self.columns;

        for r in 0..self.columns {
            let mut product_row = Vec::new();
            for c in 0..rhs.rows {
                let mut entry = 0.;
                for n in 0..mid {
                    entry += self[r][n] * rhs[n][c];
                }
                product_row.push(entry);
            }
            product.push(product_row);
        }

        Matrix::new(product)
    }
}

impl Mul<f32> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut product = Vec::new();

        for r in 0..self.rows {
            let mut product_row = Vec::new();
            for c in 0..self.columns {
                product_row.push(self[r][c] * rhs);
            }
            product.push(product_row);
        }
        Matrix::new(product)
    }
}

impl Mul<Matrix> for f32 {
    type Output = Matrix;
    
    fn mul(self, rhs: Matrix) -> Self::Output {
        rhs * self
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let vec = Matrix::new(vec![
            vec![rhs.x],
            vec![rhs.y],
            vec![rhs.z],
            vec![rhs.w],
        ]);

        let matrix_prod = self * vec;
        let result = Tuple::new(
            matrix_prod[0][0],
            matrix_prod[1][0],
            matrix_prod[2][0],
            matrix_prod[3][0],
        );
        result
    }
}

pub fn determinant(m: Matrix) -> f32 {
    m[0][0] * m[1][1] - m[0][1] * m[1][0]
}

pub fn submatrix(m: Matrix, r: usize, c: usize) -> Matrix {
    let mut sub = Vec::new();
    for row in 0..m.rows {
        let mut sub_row = Vec::new();
        for col in 0..m.columns {
            if r != row && c != col {
                sub_row.push(m[row][col]);
            }
        }
        if sub_row.len() > 0 {
            sub.push(sub_row);
        }
    }
    Matrix::new(sub)
}