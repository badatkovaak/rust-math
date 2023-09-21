// pub type Matrix2 = [[f64; 2]; 2];
// pub type Matrix3 = [[f64; 3]; 3];
pub type Matrix = Vec<Vec<f64>>;

pub fn scalar_mult(m: &Matrix, s: f64) -> Matrix {
    return m
        .iter()
        .map(|x| x.iter().map(|y| y * s).collect())
        .collect();
}

pub fn add(m1: &Matrix, m2: &Matrix) -> Matrix {
    return m1
        .iter()
        .zip(m2.iter())
        .map(|(x, y)| x.iter().zip(y.iter()).map(|(x1, y1)| x1 + y1).collect())
        .collect();
}
