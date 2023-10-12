pub mod complex;
pub mod equations;
pub mod matrix;
<<<<<<< Updated upstream
pub mod matrix2;
pub mod utils;
pub mod vector;

// use matrix::Matrix;
=======
pub mod misc;
pub mod utils;
pub mod vector;

use crate::matrix::Matrix;
>>>>>>> Stashed changes

fn main() {
    // let vec1: Vec<f64> = vec![1., 2., 3.];
    // let vec2: Vec<f64> = vec![4., 5., 6.];
    // let vec12 = vector::add(&vec1, &vec2);
    // let s1 = utils::sqrt_doom(2_000_000_000_000.0);
    // let s2 = utils::sqrt_force(2_000_000_000_000.0);
    // let s3 = utils::one_over_sqrt(2000000.0);
<<<<<<< Updated upstream
    // let m1 = vec![vec![1., 2.], vec![3., 4.]];
    let m2 = matrix::get_id_matrix(3);
    // let x = matrix::scalar_mult(&m1, 3.);
    // println!("{} {} {}", s2, 1, s1);
    println!("{:?}", m2.append_row(vec![1.0, 2.0, 3.0]));
    // println!("{:?}", matrix::add(&m1, &x));
=======
    let m1 = Matrix(vec![vec![1., 2.], vec![3., 4.]]);
    let x = Matrix::scalar_mult(m1.clone(), 3.);
    // println!("{} {} {}", s2, 1, s1);
    // println!("{:?}", m1.append_row());
    let mut a: [Vec<u32>; 3] = [vec![5, 4, 3, 2, 1], vec![], vec![]];
    misc::hanoi::myprint(&a);
    misc::hanoi::solve(&mut a, 5, 1, 2);
>>>>>>> Stashed changes
    // println!("{:?}", vector::scalar_mult(&vec1, 10.));
}
