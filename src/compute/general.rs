use crate::{
    constants::{LN2, PI},
    utils::{agm, factorial_f64},
};

pub fn ln1p(x: f64) -> f64 {
    let mut res = 0.0;
    for i in 0..50 {
        res += ((-1. as f64).powi(i) * x.powi(i + 1)) / ((i + 1) as f64);
    }
    //println!("{}", res);
    res
}

pub fn ln_newton(x: f64) -> f64 {
    let mut res = x / 1000.;
    for _i in 0..10 {
        res = res + 2. * (x - exp(res)) / (x + exp(res));
    }
    res
}

pub fn ln_app(x: f64) -> f64 {
    let m: i32 = 10;
    let s: f64 = x * ((2.0 as f64).powi(m));
    PI / (2. * agm(1., 4. / s)) - (m as f64) * LN2
}

pub fn ln(x: f64) -> f64 {
    let ln10: f64 = 2.30258_50929_94045_68401;
    let mut count: u64 = 0;
    let mut y = x;
    while y.abs() > 1. {
        y /= 10.;
        count += 1;
    }
    ln1p(y - 1.0) + (count as f64) * ln10
}

pub fn logn(x: f64, b: f64) -> f64 {
    ln(x) / ln(b)
}

pub fn exp(x: f64) -> f64 {
    let mut res = 0.0;
    let prec = 40;
    for i in 0..=prec {
        res += (x.powi(i)) / factorial_f64(i as u64);
        // println!("{}", res);
    }
    //println!("{}", res);
    res
}

pub fn expm1(x: f64) -> f64 {
    let mut res = 0.0;
    for i in 1..=40 {
        res += (x.powi(i)) / factorial_f64(i as u64);
        // println!("{}", res);
    }
    //println!("{}", res);
    res
}
