#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod algebra;
pub mod constants;
pub mod cs;
pub mod linear_algebra;
pub mod long;
pub mod numeric;
pub mod qoi;
pub mod utils;

use std::path::Path;

use linear_algebra::matrix::Matrix;

use crate::constants::{E, LN2};
use crate::numeric::general::*;
// use crate::complex_nums;
// use crate::compute;
// use crate::long;
use crate::linear_algebra::matrix_v2::MatrixV2;
use crate::long::big_digit::BigDigit;
use crate::long::big_uint::BigUInt;
// use crate::misc;
use algebra::c_algebraic::CAlg;
use constants::PI;
// use crate::long::{big_int::BigInt, big_string::BigString};
use algebra::poly_c::{gcd, PolyC};
use qoi::encoder::encode;

fn main() {
    println!();
    // let vec1: Vec<f64> = vec![1., 2., 3.];
    // let vec2: Vec<f64> = vec![4., 5., 6.];
    // let vec12 = vector::add(&vec1, &vec2);

    // let mut m1 = Matrix(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
    // let mut m2 = Matrix(vec![vec![2., 3., 4.], vec![4., 5., 6.], vec![7., 8., 9.]]);
    // let mut m3 = Matrix(vec![vec![1., 2.], vec![3., 4.]]);
    // let mut m4 = Matrix(vec![vec![1., 0.], vec![0., 1.0]]);
    // let x = Matrix::scalar_mult(m1.clone(), 3.);
    // println!(
    //     "{}\n{}\n{}\n{}\n{}",
    //     (m1.clone() + m2.clone()),
    //     (m1.clone() - m2.clone()),
    //     (m1.clone() * m2.clone()).unwrap(),
    //     m1.clone().make_into_step_form(),
    //     m2.clone().make_into_step_form()
    // );

    // let c1 = CAlgebraic(1., 1.);
    // let c2 = CAlgebraic(-1., 1.);
    //
    // println!("{}", c1 + c2);
    // println!("{}", c1 - c2);
    // println!("{}", c1 * c2);
    // println!("{}", c1 / c2);

    // println!("{}", exp(1.));
    // println!("{}", E);
    // println!("{}", ln1p(0.5));
    // println!("{}", LN2);
    // // println!("{}", ln(2.));
    // println!("{}", ln_app(2.));
    // println!("{}", ln_newton(2.));

    // let m1 = MatrixV2 {
    //     m: vec![1., 2., 3., 4.],
    //     dims: (2, 2),
    // };
    // let m2 = MatrixV2 {
    //     m: vec![2., 3., 4., 5.],
    //     dims: (2, 2),
    // };
    // let m3 = MatrixV2 {
    //     m: vec![1., 2., 3., 4., 5., 6., 7., 8., 9.],
    //     dims: (3, 3),
    // };
    // println!(
    //     "{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
    //     (&m1 + &m2).unwrap(),
    //     (&m1 - &m2).unwrap(),
    //     (&m1 * &m2).unwrap(),
    //     m1.clone().make_into_step_form(),
    //     m3.clone().make_into_step_form()
    // );

    // let p1 = Polynomial(vec![
    //     CAlg(5., 0.),
    //     CAlg(6., 0.),
    //     CAlg(7., 0.),
    //     CAlg(4., 0.),
    //     CAlg(1., 0.),
    // ]);
    // let p1 = Polynomial(vec![CAlg(4., 0.), CAlg(3., 0.), CAlg(4., 0.), CAlg(1., 0.)]);
    // let p1 = Polynomial(vec![CAlg(4., 0.), CAlg(3., 0.), CAlg(3., 0.)]);
    // let p2 = Polynomial(vec![CAlg(0., 0.), CAlg(1., 0.), CAlg(1., 0.)]);
    // let (p3, p4) = p1.clone() / p2.clone();
    // println!("{}", p2 + Polynomial(vec![CAlg(10., 0.)]));
    // println!("{}\n{}", p3, p4);

    // let p1 = PolyC(vec![CAlg(6., 0.), CAlg(7., 0.), CAlg(1., 0.)]);
    // let p2 = PolyC(vec![CAlg(-6., 0.), CAlg(-5., 0.), CAlg(1., 0.)]);
    // println!("{}", gcd(p1, p2));

    // println!(
    //     "{:?}",
    //     helper(vec![CAlg(2., 0.), CAlg(3., 0.), CAlg(4., 0.)], false)
    // );
    // println!(
    // "{:?}",
    // fft_power2(
    //     vec![CAlg(1., 0.), CAlg(2., 0.), CAlg(3., 0.), CAlg(5., 0.)],
    //     false
    // )
    // );

    // println!(
    //     "{}",
    //     compute::integration::simpsons_rule(0., 2., &(|x| compute::trig::cos(x)))
    // );
    // println!(
    //     "{}",
    //     compute::integration::trapezoidal(0., 2., &(|x| compute::trig::cos(x)))
    // );

    // let b1 = BigUInt(vec![BigDigit(0), BigDigit(11), BigDigit(2)]);
    // let b2 = BigUInt(vec![BigDigit(10), BigDigit(1), BigDigit(1)]);
    // let b1 = BigInt::construct("123".to_string());
    // let b2 = BigInt(vec![130, 3]);

    // println!();
    // println!("{:?}", b1);
    // println!("{:?}", b2);
    // // println!("{:?}", BigDigit(10) < BigDigit(9));
    // // println!("{:?}", b1.clone() + b2.clone());
    // println!("{:?}", b1.clone() - b2.clone());
    // // println!("{:?}", b2.clone() - b1.clone());

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
    println!("{:?}", encode(&Path::new("assets/out.raw")));

    println!()
}
