#[derive(Debug, Clone, Copy)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Mult,
    Div,
    Power,
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Int(i64),
    Float(f64),
    Id(char),
}
