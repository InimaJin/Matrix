/* File for testing purposes */

use matrix::Matrix;

fn main() {
    let m1 = Matrix::from_vec( vec![vec![1.0,2.0,3.0], vec![4.0,5.0,6.0], vec![7.0,8.0,9.0]] ).unwrap();  
    let m2 = Matrix::from_vec( vec![vec![3.45,2.1], vec![4.99,3.7], vec![5.32,6.8]] ).unwrap();
    let v1 = Matrix::from_vec( vec![vec![5.0], vec![3.0], vec![2.0]] ).unwrap();

    let m1_m2 = m1 * m2;
    println!("{m1_m2}");
}
