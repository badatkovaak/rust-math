pub type Vector = Vec<f64>;

pub fn scalar_mult(v: &Vector, s: f64) -> Vector {
    let mut result: Vec<f64> = Vec::new();
    for item in v.iter() {
        result.push(item * s as f64);
    }
    return result;
}

pub fn add(v1: &Vector, v2: &Vector) -> Vector {
    return v1.iter().zip(v2.iter()).map(|(x, y)| x + y).collect();
}

pub fn dot_product(v1: &Vector, v2: &Vector) -> f64 {
    return v1.iter().zip(v2.iter).map(|(x,y)| x*y ).sum()
}