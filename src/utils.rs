// pub mod utils {

pub fn one_over_sqrt(n: f64) -> f64 {
    let mut i: u64 = unsafe { std::mem::transmute::<_, u64>(n) };
    i = 0x5FE6EB50C7B537A9 - (i >> 1);
    let mut y: f64 = unsafe { std::mem::transmute(i) };
    y = y * (1.5 - ((n * 0.5) * y * y));
    y = y * (1.5 - ((n * 0.5) * y * y));

    return y;
}

pub fn sqrt_doom(n: f64) -> f64 {
    let mut i: u64 = unsafe { std::mem::transmute::<_, u64>(n) };
    i = 0x1FF7A7EF9DB22D0E + (i >> 1);
    let mut y: f64 = unsafe { std::mem::transmute(i) };

    for _i in 0..=4 {
        y = (y + n / y) / 2.0;
    }

    return y;
}

pub fn sqrt_force(n: f64) -> f64 {
    let mut res: f64 = n;
    for _i in 0..=20 {
        res = (res + n / res) / 2.0;
    }
    return res;
}

pub fn fequals(x: f64, y: f64, diff: Option<f64>) -> bool {
    f64::abs(x - y) <= diff.unwrap_or(0.000001)
}
