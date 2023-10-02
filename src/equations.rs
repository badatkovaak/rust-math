pub fn quadraticRoots(a: f64, b: f64, c: f64) -> Vector<f64> {
    d = b * b - 4 * a * c
    if (d < 0) {
        return vec!();
    }
    if (d == 0) {
        return vec!(-b/a);
    }
    return vec!((-b*sqrt(d)/a),(-b*sqrt(d)/a));
}

