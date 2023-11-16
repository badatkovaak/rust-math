#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
pub mod complex_nums;
pub mod compute;
pub mod equations;
pub mod long;
pub mod matrix;
pub mod matrix2;
pub mod misc;
pub mod quaternion;
pub mod sorting;
pub mod utils;
pub mod vector;
pub mod vector_gen;

use matrix::Matrix;

use crate::long::big_digit::BigDigit;
use crate::long::big_uint::BigUInt;
// use crate::long::{big_int::BigInt, big_string::BigString};

fn main() {
    // let vec1: Vec<f64> = vec![1., 2., 3.];
    // let vec2: Vec<f64> = vec![4., 5., 6.];
    // let vec12 = vector::add(&vec1, &vec2);

    // let mut m1 = Matrix(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
    // let mut m2 = Matrix(vec![vec![2., 3., 4.], vec![4., 5., 6.], vec![7., 8., 9.]]);
    // let mut m3 = Matrix(vec![vec![1., 2.], vec![3., 4.]]);
    // let mut m4 = Matrix(vec![vec![1., 0.], vec![0., 1.0]]);
    // let x = Matrix::scalar_mult(m1.clone(), 3.);

    let b1 = BigUInt(vec![BigDigit(0), BigDigit(11), BigDigit(2)]);
    let b2 = BigUInt(vec![BigDigit(10), BigDigit(1), BigDigit(1)]);
    // let b1 = BigInt::construct("123".to_string());
    // let b2 = BigInt(vec![130, 3]);

    println!();
    println!("{:?}", b1);
    println!("{:?}", b2);
    // println!("{:?}", BigDigit(10) < BigDigit(9));
    // println!("{:?}", b1.clone() + b2.clone());
    println!("{:?}", b1.clone() - b2.clone());
    // println!("{:?}", b2.clone() - b1.clone());

    // println!("{:?}", b1.clone() == b2.clone());
    // println!(
    //     "{:?} \n{} {} {}",
    //     &b1 + &b2,
    //     (&b1 + &b2).to_decimal_u(),
    //     b1.to_decimal_u(),
    //     (-b2).to_decimal_u()
    // );
    // let v1 = vec![1,2,3].pop()
    // let v1 = vec![9, 5, 2, 2, 6, 1, 1, 0, 2, 7];
    // let bs1 = BigString::construct(String::from("1234"));
    // let mut v1: Vec<u64> = vec![0; 10];
    // sorting::fill_random_u64(&mut v1);
    // println!("{:?}", v1);
    // println!("{:?}", sorting::bubble_sort(&v1));
    // println!(
    //     "{:?}",
    //     bs1 // String::from("1234")
    //         //     .chars()
    //         //     .map(|x| ((x as u8) < 58) && ((x as u8) > 47))
    //         //     .filter(|x| *x)
    //         //     .collect::<Vec<bool>>()
    // );

    // println!("{} {} {}", s2, 1, s1);
    // println!("{}", m1.count_elems());

    // println!("{}", m2.make_into_step_form());
    // println!("{} \n {}", m2.clone(), m2.make_into_step_form());
    // println!("{:?}", m1.append_row(vec![5.0, 6.0, 10.]));
    // let mut a: [Vec<u32>; 3] = [vec![5, 4, 3, 2, 1], vec![], vec![]];
    // misc::hanoi::myprint(&a);
    // misc::hanoi::solve(&mut a, 5, 1, 2);
    // println!("{:?}", vector::scalar_mult(&vec1, 10.));
    println!()
}
