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
