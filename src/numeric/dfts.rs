use crate::algebra::c_algebraic::CAlg;
use crate::constants::PI;
use crate::utils::is_power_of_n;

pub fn dft(values: Vec<CAlg>) -> Vec<CAlg> {
    let n = values.len();
    let res = vec![CAlg(0., 0.); n];
    let mut sum;
    let a = f64::sin_cos(2. * PI / (n as f64));
    let omega = CAlg(a.1, a.0);

    for i in 0..n {
        sum = CAlg(0., 0.);
        for j in 0..n {
            sum = sum + values[j] * omega.pow(-((i * j) as i64));
        }
    }
    res
}

pub fn fft_power2(coefs: Vec<CAlg>, is_inverse: bool) -> Option<Vec<CAlg>> {
    if coefs.len() == 1 {
        return Some(coefs);
    }

    if !is_power_of_n(coefs.len(), 2) {
        return None;
    }

    println!("\n{:?}", coefs);

    let n = coefs.len();

    let mut p_e: Vec<CAlg> = vec![];
    let mut p_o: Vec<CAlg> = vec![];

    for i in 0..n {
        if i % 2 == 0 {
            p_e.push(coefs[i]);
        } else {
            p_o.push(coefs[i]);
        }
    }

    let omega: CAlg;
    let a = f64::sin_cos(2. * PI / (n as f64));
    println!("{:?}", a);
    if !is_inverse {
        omega = CAlg(a.1, a.0);
    } else {
        omega = (-CAlg(a.1, a.0)).scale(1. / (n as f64));
    }
    println!("{}", omega);

    let y_e = fft_power2(p_e, is_inverse).unwrap();
    let y_o = fft_power2(p_o, is_inverse).unwrap();
    let mut res = vec![CAlg(0., 0.); n];

    for i in 0..n / 2 {
        println!("{}, {}", i, omega.pow(i as i64));
        res[i] = y_e[i] + omega.pow(i as i64) * y_o[i];
        res[i + n / 2] = y_e[i] - omega.pow(i as i64) * y_o[i];
    }

    println!("{:?}\n", res);
    Some(res)
}
