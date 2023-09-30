// pub mod utils {

pub fn one_over_sqrt(n: f64) -> f64 {
    let mut i: u64 = unsafe { std::mem::transmute::<_, u64>(n) };
    i = 0x5FE6EB50C7B537A9 - (i >> 1);
    let mut y: f64 = unsafe { std::mem::transmute(i) };
    y = y * (1.5 - ((n * 0.5) * y * y));
    y = y * (1.5 - ((n * 0.5) * y * y));

    return y;
}
//
// pub fn sqrt_doom(n: f64) -> f64 {
//     let mut i: u64 = unsafe { std::mem::transmute::<_, u64>(n) };
//     i = 0x5FE6EB50C7B537A9 + (i >> 1);
//     let mut y: f64 = unsafe { std::mem::transmute(i) };
//     for _i in 0..=5 {
//         y = y * (1.5 - ((n * 0.5) * y * y));
//     }
//     return y;
// }

pub fn sqrt_force(n: f64) -> f64 {
    let mut res: f64 = n;
    for _i in 0..=10 {
        res = 0.5 * (res + n / res);
    }
    return res;
}
