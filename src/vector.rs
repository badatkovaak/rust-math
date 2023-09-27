// use mymath::utils;

pub type Vector = Vec<f64>;

pub fn scalar_mult(v: &Vector, s: f64) -> Vector {
    let mut result: Vec<f64> = Vec::new();
    for item in v.iter() {
        result.push(item * s as f64);
    }
    return result;
}

pub fn get_length(v: &Vector) -> f64 {
    return f64::sqrt(v.iter().map(|x| x * x).sum());
}

pub fn add(v1: &Vector, v2: &Vector) -> Vector {
    return v1.iter().zip(v2.iter()).map(|(x, y)| x + y).collect();
}

pub fn dot_product(v1: &Vector, v2: &Vector) -> f64 {
    return v1.iter().zip(v2.iter()).map(|(x, y)| x * y).sum();
}

pub fn normalize(v: &Vector) -> Vector {
    // return scalar_mult(v, 1. / get_length(v));
    return scalar_mult(v, crate::utils::one_over_sqrt(get_length(v)));
}

// pub fn cross_product(v1: &Vector, v2: &Vector) -> Vector {
//
// }
