pub mod complex_nums;
pub mod equations;
pub mod matrix;
pub mod matrix2;
pub mod misc;
pub mod utils;
pub mod vector;
pub mod vector_gen;

use matrix::Matrix;
fn main() {
    // let vec1: Vec<f64> = vec![1., 2., 3.];
    // let vec2: Vec<f64> = vec![4., 5., 6.];
    // let vec12 = vector::add(&vec1, &vec2);
    // let s1 = utils::sqrt_doom(2_000_000_000_000.0);
    // let s2 = utils::sqrt_force(2_000_000_000_000.0);
    // let s3 = utils::one_over_sqrt(2000000.0);
    let m1 = Matrix(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
    // let x = Matrix::scalar_mult(m1.clone(), 3.);
    // println!("{} {} {}", s2, 1, s1);

    println!("{:?} \n {}", m1, m1);
    // println!("{:?}", m1.append_row(vec![5.0, 6.0, 10.]));
    // let mut a: [Vec<u32>; 3] = [vec![5, 4, 3, 2, 1], vec![], vec![]];
    // misc::hanoi::myprint(&a);
    // misc::hanoi::solve(&mut a, 5, 1, 2);
    // >>>>>>> Stashed changes
    // println!("{:?}", vector::scalar_mult(&vec1, 10.));
}
