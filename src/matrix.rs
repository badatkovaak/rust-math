pub type Matrix2 = [[f64; 2]; 2];
// pub type Matrix3 = [[f64; 3]; 3];
pub type Matrix = Vec<Vec<f64>>;

pub fn scalar_mult(m: &Matrix, s: f64) -> Matrix {
    return m
        .iter()
        .map(|x| x.iter().map(|y| y * s).collect())
        .collect();
}

pub fn scalar_mult2(m1: &Matrix2, s: f64) -> Matrix2 {
    return m.map(|x| x.map(|y| y * s ))
}

pub fn add2(m1: &Matrix2 , m2: &Matrix2) -> Matrix2 {
    return m1
        .iter()
        .zip(m2.iter())
        .map(|(x,y)| x.iter().zip(y.iter()).map(|(x1,y1)| x1 + y1).collect::<Vec<f64>>().try_into().unwrap())
        .collect::<Vec<[f64;2]>>()
        .try_into()
        .unwrap()
}

pub fn add(m1: &Matrix, m2: &Matrix) -> Matrix {
    return m1
        .iter()
        .zip(m2.iter())
        .map(|(x, y)| x.iter().zip(y.iter()).map(|(x1, y1)| x1 + y1).collect())
        .collect();
}
