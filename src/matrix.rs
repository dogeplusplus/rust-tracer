use std::ops::{Index, Mul};

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