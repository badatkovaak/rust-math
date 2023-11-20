use crate::{constants::PI, utils::factorial_f64};

pub fn sin(x: f64) -> f64 {
    let mut res = x;
    let y = trim_2pi(x);
    for i in 1..=20 {
        res += (((-1.0 as f64).powi(i)) * (y.powi(2 * i + 1))) / factorial_f64((2 * i + 1) as u64);
    }
    //println!("{}", res);
    res
}

pub fn cos(x: f64) -> f64 {
    let mut res = 1.0;
    let y = trim_2pi(x);
    for i in 1..=20 {
        res += (((-1.0 as f64).powi(i)) * (y.powi(2 * i))) / factorial_f64((2 * i) as u64);
    }
    //println!("{}", res);
    res
}

#[inline]
pub fn trim_2pi(x: f64) -> f64 {
    let mut y = x;
    while y.abs() > 2. * PI {
        if y > 0. {
            y -= 2. * PI;
        } else {
            y += 2. * PI;
        }
    }
    return y;
}
