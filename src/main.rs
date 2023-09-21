// use crate::vector;
pub mod matrix;
pub mod vector;

fn main() {
    let vec1: Vec<f64> = vec![1., 2., 3.];
    let vec2: Vec<f64> = vec![4., 5., 6.];
    let vec12 = vector::add(&vec1, &vec2);

    let m1 = vec![vec![1., 2.], vec![3., 4.]];
    // let m2 =
    let x = matrix::scalar_mult(&m1, 3.);
    println!("{:?}", matrix::add(&m1, &x));
    println!("{:?}", vector::scalar_mult(&vec1, 10.));
}
