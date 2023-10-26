pub mod arithmetic;
pub mod complex_nums;
pub mod equations;
pub mod matrix;
pub mod matrix2;
pub mod misc;
pub mod sorting;
pub mod utils;
pub mod vector;
pub mod vector_gen;

use matrix::Matrix;

use crate::arithmetic::big_int::BigInt;

fn main() {
    // let vec1: Vec<f64> = vec![1., 2., 3.];
    // let vec2: Vec<f64> = vec![4., 5., 6.];
    // let vec12 = vector::add(&vec1, &vec2);

    // let s1 = utils::sqrt_doom(2_000_000_000_000.0);
    // let s2 = utils::sqrt_force(2_000_000_000_000.0);
    // let s3 = utils::one_over_sqrt(2000000.0);

    // let mut m1 = Matrix(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
    let mut m2 = Matrix(vec![vec![2., 3., 4.], vec![4., 5., 6.], vec![7., 8., 9.]]);
    // let mut m3 = Matrix(vec![vec![1., 2.], vec![3., 4.]]);
    // let mut m4 = Matrix(vec![vec![1., 0.], vec![0., 1.0]]);
    // let x = Matrix::scalar_mult(m1.clone(), 3.);

    // let b1 = BigInt(vec![240, 4]);
    // let b2 = BigInt(vec![130, 3]);
    // println!(
    //     "{:?} \n{} {} {}",
    //     &b1 + &b2,
    //     (&b1 + &b2).to_decimal_u(),
    //     b1.to_decimal_u(),
    //     (-b2).to_decimal_u()
    // );
    // let v1 = vec![9, 5, 2, 2, 6, 1, 1, 0, 2, 7];
    let mut v1: Vec<u64> = vec![0; 10];
    sorting::fill_random_u64(&mut v1);
    println!("{:?}", v1);
    println!("{:?}", sorting::bubble_sort(&v1));

    // println!("{} {} {}", s2, 1, s1);
    // println!("{}", m1.count_elems());

    // println!("{}", m2.make_into_step_form());
    // println!("{} \n {}", m2.clone(), m2.make_into_step_form());
    // println!("{:?}", m1.append_row(vec![5.0, 6.0, 10.]));
    // let mut a: [Vec<u32>; 3] = [vec![5, 4, 3, 2, 1], vec![], vec![]];
    // misc::hanoi::myprint(&a);
    // misc::hanoi::solve(&mut a, 5, 1, 2);
    // >>>>>>> Stashed changes
    // println!("{:?}", vector::scalar_mult(&vec1, 10.));
}
