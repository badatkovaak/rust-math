pub fn trapezoidal(a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
    let n = ((b - a).ceil() as u64) * 10000;
    println!("{}", n);
    ((b - a) / (n as f64))
        * ((f(a) / 2.)
            + (1..(n - 1))
                .map(|k| f(a + (k as f64) * (b - a) / (n as f64)))
                .sum::<f64>()
            + (f(b) / 2.))
}

pub fn simpsons_rule(a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
    let n = ((b - a).ceil() as u64) * 10000;
    println!("{}", n);
    let h = (b - a) / (n as f64);

    h / 3.
        * (f(a)
            + 4. * (1..(n / 2))
                .map(|i| f(a + ((2 * i - 1) as f64) * h))
                .sum::<f64>()
            + 2. * (1..(n / 2 - 1))
                .map(|i| f(a + ((2 * i) as f64) * h))
                .sum::<f64>()
            + f(b))
}
