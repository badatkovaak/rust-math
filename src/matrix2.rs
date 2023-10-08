pub type Matrix2 = [[f64; 2]; 2];

pub fn scalar_mult2(m: &Matrix2, s: f64) -> Matrix2 {
    return m.map(|x| x.map(|y| y * s));
}

pub fn add2(m1: &Matrix2, m2: &Matrix2) -> Matrix2 {
    return m1
        .iter()
        .zip(m2.iter())
        .map(|(x, y)| {
            x.iter()
                .zip(y.iter())
                .map(|(x1, y1)| x1 + y1)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[f64; 2]>>()
        .try_into()
        .unwrap();
}
