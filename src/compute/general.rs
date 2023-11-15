use crate::utils::factorial_f64;

pub fn lnp1(x: f64) -> f64 {
    let mut res = 0.0;
    for i in 0..20 {
        res += ((-1. as f64).powi(i) * x.powi(i + 1)) / ((i + 1) as f64);
    }
    //println!("{}", res);
    res
}

// pub fn ln(x:f64) {}

//pub fn ln(x: f64) -> f64 {
//    // let mut res = 0.0;
//    let ln10: f64 = 2.30258509299;
//    let mut count:u64 = 0;
//    let mut y = x;
//    while y.abs() > 1. {
//        y /= 10.;
//        count += 1;
//    }
//    lnp1(y - 1.0) + (count as f64) * ln10
//}

// pub fn logn(x: f64, b: f64) -> f64 {
//     ln(x) / ln(b)
// }

pub fn exp(x: f64) -> f64 {
    let mut res = 0.0;
    let prec = 20;
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
