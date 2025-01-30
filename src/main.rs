/* File for testing purposes */

use matrix::Matrix;

fn main() {
    //Create a 3x3 matrix:
    let mut a: Matrix<f64> = Matrix::from_vec(vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ])
    .unwrap();

    println!("{a}\n");
    a.transpose();
    println!("{a}");
}
