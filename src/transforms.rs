use crate::matrix::Matrix;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix<f32, 4, 4> {
    let trans_mat = Matrix::new([
        [1., 0., 0., x],
        [0., 1., 0., y],
        [0., 0., 1., z],
        [0., 0., 0., 1.],
    ]);
    trans_mat
}