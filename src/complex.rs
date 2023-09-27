// use crate::utils;

pub type ComplexAlg = (f64, f64);
pub type ComplexPolar = (f64, f64);

pub fn add(c1: ComplexAlg, c2: ComplexAlg) -> ComplexAlg {
    return (c1.0 + c2.0, c1.1 + c2.1);
}

pub fn conjugate(c: &ComplexAlg) -> ComplexAlg {
    return (c.0, -c.1);
}

pub fn algebraic_to_polar(c: &ComplexAlg) -> ComplexPolar {
    let r = f64::sqrt(c.0 * c.0 + c.1 * c.1);
    // let one_r = utils::one_over_sqrt(c.0 * c.0 + c.1 * c.1);
    return (c.0 / r, c.1 / r);
    // return (c.0 * one_r, c.1 * one_r);
}

// pub fn polar_to_algebraic(c: &ComplexPolar) -> ComplexAlg {
//     return ();
// }
