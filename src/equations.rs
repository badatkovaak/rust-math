use crate::utils;

pub fn quadratic_roots(a: f64, b: f64, c: f64) -> Vec<f64> {
    let d: f64 = b * b - 4.0 * a * c;
    if d < 0.0 {
        return vec![];
    }
    if d == 0.0 {
        return vec![-b / a];
    }
    return vec![
        (-b * utils::sqrt_force(d) / a),
        (-b * utils::sqrt_force(d) / a),
    ];
}

// pub fn cubic_roots(a: f64, b: f64, c: f64, d: f64) -> Vec<f64> {
//     let d0 = b*b - 3. * a * c;
//     let d1 = 2. * b*b*b - 9.*a*b*c + 27.*a*a*a*d;
//     let res = vec![];
//     res
// }
