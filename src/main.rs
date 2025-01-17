/* File for testing purposes */

use matrix::Matrix;

fn main() {
    let m1 = Matrix::from_vec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).unwrap();
    let m2 = Matrix::from_vec(vec![vec![3, 2,1], vec![4, 3,7], vec![5, 6,-4]]).unwrap();

    let m1_m2 = m2 - m1;
    println!("{m1_m2}");
    
    let m3 = Matrix::from_vec(vec![
        vec![-8.0, 3.0, 29.0],
        vec![300.0, 92.0, 7.4],
        vec![3.0, -145.0, 0.0],
    ])
    .unwrap();
    println!("{}", m3.det().unwrap());
}
