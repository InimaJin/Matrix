use matrix::Matrix;

fn main() {
    let v = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let mut m2 = Matrix::from_vec(v).unwrap();
    m2 *= 3.0;
    println!("{m2}");
}
