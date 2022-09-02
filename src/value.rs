#[derive(PartialEq, Debug)]
pub enum Value {
    Null,
    Bool(bool),
    String(String),
    I64(i64),
    F64(f64),
}
