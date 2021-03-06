use crate::{cross, matrix::Matrix, normalize, Tuple};

pub fn translation(x: f32, y: f32, z: f32) -> Matrix<f32, 4, 4> {
    let trans_mat = Matrix::new([
        [1., 0., 0., x],
        [0., 1., 0., y],
        [0., 0., 1., z],
        [0., 0., 0., 1.],
    ]);
    trans_mat
}

pub fn scaling(x: f32, y: f32, z: f32) -> Matrix<f32, 4, 4> {
    let scale_mat = Matrix::new([
        [x, 0., 0., 0.],
        [0., y, 0., 0.],
        [0., 0., z, 0.],
        [0., 0., 0., 1.],
    ]);
    scale_mat
}

pub fn rotation_x(r: f32) -> Matrix<f32, 4, 4> {
    let rot_matrix_x = Matrix::new([
        [1., 0., 0., 0.],
        [0., f32::cos(r), -f32::sin(r), 0.],
        [0., f32::sin(r), f32::cos(r), 0.],
        [0., 0., 0., 1.],
    ]);
    rot_matrix_x
}

pub fn rotation_y(r: f32) -> Matrix<f32, 4, 4> {
    let rot_matrix_y = Matrix::new([
        [f32::cos(r), 0., f32::sin(r), 0.],
        [0., 1., 0., 0.],
        [-f32::sin(r), 0., f32::cos(r), 0.],
        [0., 0., 0., 1.],
    ]);
    rot_matrix_y
}

pub fn rotation_z(r: f32) -> Matrix<f32, 4, 4> {
    let rot_matrix_z = Matrix::new([
        [f32::cos(r), -f32::sin(r), 0., 0.],
        [f32::sin(r), f32::cos(r), 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ]);
    rot_matrix_z
}

pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix<f32, 4, 4> {
    let shear = Matrix::new([
        [1., xy, xz, 0.],
        [yx, 1., yz, 0.],
        [zx, zy, 1., 0.],
        [0., 0., 0., 1.],
    ]);
    shear
}

pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix<f32, 4, 4> {
    let forward = normalize(to - from);
    let upn = normalize(up);
    let left = cross(forward, upn);
    let true_up = cross(left, forward);

    let orientation = Matrix::new([
        [left.x, left.y, left.z, 0.],
        [true_up.x, true_up.y, true_up.z, 0.],
        [-forward.x, -forward.y, -forward.z, 0.],
        [0., 0., 0., 1.],
    ]);

    orientation * translation(-from.x, -from.y, -from.z)
}
