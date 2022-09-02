use crate::row::Row;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeaderWithType {
    String(String),
    Integer(String),
    Float(String),
    None,
}

#[derive(Debug)]
pub struct Header {
    pub header: Vec<HeaderWithType>,
}

impl Header {
    pub fn new(headers: Vec<HeaderWithType>) -> Option<Self> {
        if !headers.is_empty() {
            return Some(Header { header: headers });
        }

        None
    }
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub header: Option<Header>,
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new(name: &str, header: Option<Header>, rows: Vec<Row>) -> Self {
        Table {
            name: name.to_string(),
            header,
            rows,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn vale_test() {
        let _v = Value::Null;
        let _v = Value::Bool(true);
        let _v = Value::String(String::from("hello"));
        let _v = Value::I64(1);
        let _v = Value::F64(1.0);
    }

    #[test]
    fn test_table() {
        let _t = Table {
            name: String::from("test"),
            header: Some(Header {
                header: vec![
                    HeaderWithType::String(String::from("name")),
                    HeaderWithType::Integer(String::from("age")),
                    HeaderWithType::Float(String::from("height")),
                ],
            }),
            rows: vec![
                Row {
                    values: vec![
                        Value::String(String::from("hello")),
                        Value::I64(1),
                        Value::F64(1.0),
                    ],
                },
                Row {
                    values: vec![
                        Value::String(String::from("world")),
                        Value::I64(2),
                        Value::F64(2.0),
                    ],
                },
            ],
        };
    }
}
