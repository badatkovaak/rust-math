// pub mod utils {

pub fn one_over_sqrt(n: f64) -> f64 {
    let mut y: f64 = f64::from_bits(0x5f3759df - (n.to_bits() >> 1));
    // let x2: f64 = n * 0.5;
    y = y * (1.5 - ((n * 0.5) * y * y));
    y = y * (1.5 - ((n * 0.5) * y * y));
    return y;
}
// }
