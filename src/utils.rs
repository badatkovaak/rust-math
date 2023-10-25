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

#[inline]
pub fn fequals(x: f64, y: f64, diff: Option<f64>) -> bool {
    f64::abs(x - y) <= diff.unwrap_or(0.000000001)
}

macro_rules! max_of_two {
    ($x:ty, $name:tt) => {
        pub fn $name(a: $x, b: $x) -> $x {
            a * ((a >= b) as $x) + b * ((b > a) as $x)
        }
    };
}

max_of_two!(u8, max_of_two_u8);
max_of_two!(u16, max_of_two_u16);
max_of_two!(u32, max_of_two_u32);
max_of_two!(u64, max_of_two_u64);
max_of_two!(u128, max_of_two_u128);
max_of_two!(usize, max_of_two_usize);

max_of_two!(i8, max_of_two_i8);
max_of_two!(i16, max_of_two_i16);
max_of_two!(i32, max_of_two_i32);
max_of_two!(i64, max_of_two_i64);
max_of_two!(i128, max_of_two_i128);

// pub fn divmod
