/* File for testing purposes */

use matrix::Matrix;

fn main() {
    //Create a 3x3 matrix:
    let a: Matrix<f64> = Matrix::from_vec(vec![
        vec![1.0, 2.0, 3.0],
        vec![2.0, 1.0, 4.0],
        vec![7.0, -3.0, 2.0],
    ])
    .unwrap();

    let mut b = a.clone();
    b *= 2.0;

    /* 3x3 diagonal matrix:
     * [ [2, 0, 0], [0, 2, 0], [0, 0, 2] ] */
    let i_2 = Matrix::build_scalar_matrix(2.0, a.width()).unwrap();

    assert!(a * i_2 == b);
    
    println!("{b}");
    b.to_row_echelon();
    println!("{b}");
}
