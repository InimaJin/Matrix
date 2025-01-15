use matrix::Matrix;

fn main() {
    let v = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let m2 = Matrix::from_vec(v).unwrap();

    let x = m2 * 4.0;
    println!("{}", x);
}
