#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Point(Box<[f64]>);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point2(f64, f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point3(f64, f64, f64);
