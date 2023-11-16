use crate::utils::factorial_f64;

pub fn sin(x: f64) -> f64 {
    let mut res = x;
    for i in 1..=20 {
        res += (((-1.0 as f64).powi(i)) * (x.powi(2 * i + 1))) / factorial_f64((2 * i + 1) as u64);
    }
    //println!("{}", res);
    res
}

pub fn cos(x: f64) -> f64 {
    let mut res = 1.0;
    for i in 1..=20 {
        res += (((-1.0 as f64).powi(i)) * (x.powi(2 * i))) / factorial_f64((2 * i) as u64);
    }
    //println!("{}", res);
    res
}
