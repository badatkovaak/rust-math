pub fn to_chunks<T>(v: Vec<T>, n: usize) -> Vec<Vec<T>> {
    let mut res = Vec::with_capacity(n);
    res.push(vec![]);
    let mut count = 0;
    for k in v.iter() {
        if res[count].len() <= n {
            res[count].push(*k);
        } else {
            count += 1;
            res.push(vec![]);
        }
    }
    res
}

pub fn to_chunks1<T>(v: &[T], n: usize) -> Vec<Vec<T>> {
    let mut res = Vec::with_capacity(n);
    res.push(vec![]);
    let mut count = 0;
    for k in v.iter() {
        if res[count].len() <= n {
            res[count].push(*k);
        } else {
            count += 1;
            res.push(vec![]);
        }
    }
    res
}

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

pub fn agm(x: f64, y: f64) -> f64 {
    let mut a = x;
    let mut b = y;
    for _ in 0..20 {
        (a, b) = ((a + b) / 2., f64::sqrt(a * b));
    }
    (a + b) / 2.
}

pub fn gcd_euclid(x: u64, y: u64) -> u64 {
    let mut a;
    let mut b;

    if x > y {
        (a, b) = (x, y);
    } else if y > x {
        (a, b) = (y, x);
    } else {
        return 1;
    }

    while a != 0 && b != 0 {
        (a, b) = (b, a % b);
    }

    if a == 0 {
        return b;
    } else {
        return a;
    }
}

pub fn factorial(n: u64) -> u64 {
    let mut res = 1;
    for i in 2..n {
        res *= i;
    }
    res
}

pub fn factorial_f64(n: u64) -> f64 {
    let mut res = 1.0;
    for i in 2..=n {
        res *= i as f64;
    }
    // println!("{}", res);
    res
}

pub fn is_power_of_n<
    T: std::ops::Div<Output = T> + std::ops::Rem<Output = T> + std::cmp::PartialEq + Copy + From<u8>,
>(
    value: T,
    n: T,
) -> bool {
    //
    let mut res = value;
    while res % n == T::from(0) {
        res = res / value;
    }
    true
    // res
}

pub fn max_of_two<
    T: std::cmp::PartialOrd
        + From<bool>
        + std::ops::Mul<Output = T>
        + Copy
        + std::ops::Add<Output = T>,
>(
    a: T,
    b: T,
) -> T {
    a * T::from(a >= b) + b * T::from(b > a)
}

// #[inline]
pub fn fequals(x: f64, y: f64, diff: u64) -> bool {
    f64::abs(x - y) <= (10.0 as f64).powi(-(diff as i32))
}

// macro_rules! max_of_two {
//     ($x:ty, $name:tt) => {
//         pub fn $name(a: $x, b: $x) -> $x {
//             a * ((a >= b) as $x) + b * ((b > a) as $x)
//         }
//     };
// }
//
// max_of_two!(u8, max_of_two_u8);
// max_of_two!(u16, max_of_two_u16);
// max_of_two!(u32, max_of_two_u32);
// max_of_two!(u64, max_of_two_u64);
// max_of_two!(u128, max_of_two_u128);
// max_of_two!(usize, max_of_two_usize);
//
// max_of_two!(i8, max_of_two_i8);
// max_of_two!(i16, max_of_two_i16);
// max_of_two!(i32, max_of_two_i32);
// max_of_two!(i64, max_of_two_i64);
// max_of_two!(i128, max_of_two_i128);
// max_of_two!(isize, max_of_two_isize);

// pub fn divmod
