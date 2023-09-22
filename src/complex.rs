pub type complex = (f64,f64);

pub fn add(c1: Complex, c2: Complex) -> Complex {
    return (c1[0] + c2[0],c1[1] + c2[1])
}