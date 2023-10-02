pub mod complex;
pub mod equations;
pub mod matrix;
pub mod utils;
pub mod vector;

fn main() {
    // let vec1: Vec<f64> = vec![1., 2., 3.];
    // let vec2: Vec<f64> = vec![4., 5., 6.];
    // let vec12 = vector::add(&vec1, &vec2);
    let s1 = utils::sqrt_doom(2_000_000_000_000.0);
    let s2 = utils::sqrt_force(2_000_000_000_000.0);
    // let s3 = utils::one_over_sqrt(2000000.0);
    // let m1 = vec![vec![1., 2.], vec![3., 4.]];
    // let m2 =
    // let x = matrix::scalar_mult(&m1, 3.);
    println!("{} {} {}", s2, 1, s1);
    // println!("{:?}", matrix::add(&m1, &x));
    // println!("{:?}", vector::scalar_mult(&vec1, 10.));
}
