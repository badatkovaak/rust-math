#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod algebra;
pub mod constants;
pub mod cs;
pub mod exprlang;
pub mod geometry;
pub mod linear_algebra;
pub mod long;
pub mod numeric;
pub mod qoi;
pub mod utils;

use std::path::Path;
use std::time::{self, Instant};

use crate::cs::sorting;
use crate::exprlang::lexer::lex;
use crate::exprlang::poly::{poly_to_polysym, run_through};
use crate::utils::{gcd_euclid, lcm_euclid};
use linear_algebra::matrix::Matrix;
use linear_algebra::rat_matrix::RatMatrix;

use crate::constants::{E, LN2};
use crate::cs::life::set_bit;
use crate::linear_algebra::matrix_v2::MatrixV2;
use crate::long::big_digit::BigDigit;
use crate::long::big_uint::BigUInt;
use crate::numeric::general::*;
use exprlang::poly::{self, parse_poly, Parser};
// use crate::misc;
use algebra::complex::Complex;
use constants::PI;
// use crate::long::{big_int::BigInt, big_string::BigString};
use algebra::poly_c::PolyC;
use algebra::rational::Rational as R;
use algebra::sym_poly as SP;
use algebra::symbol::Symbol as S;
use cs::life::{self, Game};

fn main() {
    println!();

    // let f = SP::PolySym(vec![S(vec![('B', 1.)], 0.), S(vec![('A', 1.)], 0.)]);
    // let g = SP::PolySym(vec![S(vec![], 2.), S(vec![], 2.)]);
    // let s1 = S(vec![('A', 1.)], 1.);
    // let s2 = S(vec![('B', 1.)], 2.);
    // println!("{}", s1 + s2);
    // println!("{}\n{}\n{}", f.clone(), g.clone(), f * g);

    // let p1 = run_through("(3A)x^2 + (2B)x + C").unwrap();
    // let p2 = run_through("(A)x^3 + (B)x^2 + (C)x + D").unwrap();
    // let p3 = run_through("(E)x^3 + (F)x^2 + (G)x + H").unwrap();
    // let p4 = run_through("x^2 + 4").unwrap();
    // let p5 = run_through("x^2 + (4)x + 5").unwrap();
    // let p6 = run_through("(2)x").unwrap();
    // let p7 = run_through("(2)x + 4").unwrap();
    // let p8 = run_through("x^5 + (-1)x^4 + (-26)x^2 + (-24)x - 25").unwrap();

    // let pt1 = p1.clone() * p4.clone() * p5.clone();
    // let pt2 = p2.clone() * (p6.clone() * p5.clone() + p7.clone() * p4.clone());
    // let pt3 = p3.clone() * p4.clone() * p5.clone();
    // let res = pt1.clone() - pt2.clone() + pt3.clone() - p8.clone();

    // println!(
    //     "\n{}\n\n{}\n\n{}\n\n{}\n",
    //     pt1.clone(),
    //     pt2.clone(),
    //     pt3.clone(),
    //     res
    // );
    // println!("")
    // println!(
    //     "\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{:?}\n",
    //     p1,
    //     p2,
    //     p3,
    //     p4,
    //     p5,
    //     p6,
    //     p7,
    //     p8,
    //     res.eval(-1.)
    // );

    #[rustfmt::skip]
    let m1 = Matrix(vec![
        vec![ 4., -2.,  0.,   0.,  1.,  0.,   1.],
        vec![18., -4., -3.,   0.,  4.,  1.,  -1.],
        vec![48.,  0., -8.,  -4.,  9.,  4.,   0.],
        vec![80., 16., -9., -12., 16.,  9., -26.],
        vec![ 0., 40.,  0., -18., 20., 16., -24.],
        vec![ 0.,  0., 20., -16.,  0., 20., -25.],
    ]);
    // #[rustfmt::skip]
    // let m2 = Matrix(vec![
    //     vec![  10.,  20., -20.,   4.,  -75. / 4.],
    //     vec![  80.,  90., -60.,  -4.,       -20.],
    //     vec![  80., 240., -90., -24., -615. / 4.],
    //     vec![-400., 400., -80., -20.,      -120.],
    // ]);
    // #[rustfmt::skip]
    // let m3 = Matrix(vec![
    //     vec![  10.,  170., -92.,126.25],
    //     vec![  80.,  100., -36.,  130.],
    //     vec![  1200., -880., 140., -870.],
    // ]);
    // println!("{}", m1);
    // println!("{}", m3);
    // println!("{}", m1.to_row_echelon_form());
    // println!("{}", m1.to_id());
    // println!("{}", m2.to_row_echelon_form());
    // println!("{}", m3.to_row_echelon_form());

    #[rustfmt::skip]
    let m1 = RatMatrix {m: Into::into(vec![ 
         R(4,1), R(-2,1),  R(0,1),   R(0,1),  R(1,1),  R(0,1),   R(1,1),
        R(18,1), R(-4,1), R(-3,1),   R(0,1),  R(4,1),  R(1,1),  R(-1,1),
        R(48,1),  R(0,1), R(-8,1),  R(-4,1),  R(9,1),  R(4,1),   R(0,1),
        R(80,1), R(16,1), R(-9,1), R(-12,1), R(16,1),  R(9,1), R(-26,1),
         R(0,1), R(40,1),  R(0,1), R(-18,1), R(20,1), R(16,1), R(-24,1),
         R(0,1),  R(0,1), R(20,1), R(-16,1),  R(0,1), R(20,1), R(-25,1),
    ]), dims: (6,7)};
    // println!("{}\n", m1);
    // println!("{}", m1.to_row_echelon_form());

    // let a = 10;
    // let b = 0;
    // println!("{} {}", lcm_euclid(a, b), gcd_euclid(a, b));
    // let pt1 = (p1.clone() * p3.clone() * p4.clone()).sym_pretti();
    // let pt2 = -p2.clone() * p5;
    // let pt3 = (p6 * p3.pow(3) * res  println!("{}", p2.eval(2.));

    // println!("{}\n{}\n{}", pt1, pt2, pt3);
    // let ans = p1 * p3.clone() * p4.clone() - p2 * p5 + p6 * p3.pow(3) * p4.pow(3);
    // println!("ans : {}", pt1 + pt2 + pt3);

    // let l = poly::lex("(10 - a + 20)x^3 + (b - c)x^2 + 20");
    // println!("{:?}", l);
    // let mut par = Parser {
    //     toks: l.unwrap(),
    //     pos: 0,
    // };
    // let p = parse_poly(&mut par).unwrap();
    // let pol = poly_to_polysym(p);
    // println!("{}", pol);

    // let l = lex("10 + 3 - 27 * 2.5 / 0.6 - .7  * - 1. < > <= >= == = ( )");
    // println!("{:?}", l.unwrap());

    // let vec1: Vec<f64> = vec![1., 2., 3.];
    // let vec2: Vec<f64> = vec![4., 5., 6.];
    // let vec12 = vector::add(&vec1, &vec2);

    // let mut m1 = Matrix(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
    // let mut m2 = Matrix(vec![vec![2., 3., 4.], vec![4., 5., 6.], vec![6., 8., 9.]]);
    // let mut m3 = Matrix(vec![vec![1., 2.], vec![3., 4.]]);
    // let mut m4 = Matrix(vec![vec![1., 0.], vec![0., 1.0]]);
    // let x = Matrix::scalar_mult(m1.clone(), 3.);
    // println!(
    //     "{}\n{}\n{}\n{}\n{}",
    //     (m1.clone() + m2.clone()),
    //     (m1.clone() - m2.clone()),
    //     (m1.clone() * m2.clone()).unwrap(),
    //     // m1.clone().to_row_echelon_form(),
    //     m2.clone().to_row_echelon_form(),
    //     m2.clone().inverse()
    // );

    // let (h, w) = (1024, 1024);
    // let z = vec![17, 19, 34, 35, 50];
    // let z = vec![3, 35, 67];
    // let z = vec![
    //     (4, 6),
    //     (5, 6),
    //     (6, 5),
    //     (6, 7),
    //     (7, 6),
    //     (8, 6),
    //     (9, 6),
    //     (10, 6),
    //     (11, 5),
    //     (11, 7),
    //     (12, 6),
    //     (13, 6),
    // ];
    // let start = time::Instant::now();
    // let mut x = Game::new_from_coords(z, h, w).unwrap();
    // println!("{}\n", x.clone());
    // let n = 300420392;
    // println!(
    //     "{}\n{}",
    //     life::to_binary_string(n),
    //     life::to_binary_string(set_bit(n, 20, false))
    // );
    // for i in 0..20 {
    // x.run();
    // println!("\n{}", x.clone());
    // }

    // let end = start.elapsed();
    // println!("Executed in  : {:?}", end);
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
    // let mut v1: Vec<u64> = vec![0; 15];
    // sorting::fill_random_u64(&mut v1, 10);
    // println!("{:?}", v1);
    // println!("{:?}", sorting::merge_sort(&v1));
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
    // println!("{:?}", encode(&Path::new("assets/out.raw")));

    println!()
}
