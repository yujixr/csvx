use super::*;
use std::fmt;

/// Value types of CSVX items
#[derive(Clone)]
pub enum Value {
    Error,
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Ref(usize, usize),
    Empty,
}

impl Node for Value {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<dyn Node>, Vec<(usize, usize)>) {
        let val = &seqs[0][0];
        match val.to_owned() {
            Token::Integer(x) => (Box::new(Value::Integer(x)), vec![]),
            Token::Float(x) => (Box::new(Value::Float(x)), vec![]),
            Token::String(x) => (Box::new(Value::String(x)), vec![]),
            Token::Boolean(x) => (Box::new(Value::Boolean(x)), vec![]),
            Token::Ref(x, y) => (Box::new(Value::Ref(x, y)), vec![(x, y)]),
            _ => (Box::new(Value::Error), vec![]),
        }
    }
    fn calc(&self, calculated_table: &Vec<Vec<Value>>) -> Value {
        if let &Value::Ref(x, y) = self {
            if y < calculated_table.len() && x < calculated_table[y].len() {
                return calculated_table[y][x].clone();
            } else {
                Value::Error
            }
        } else {
            self.clone()
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(x) => write!(f, "{}", x),
            Value::Float(x) => write!(f, "{}", x),
            Value::String(x) => write!(f, "\"{}\"", x),
            Value::Boolean(x) => write!(f, "{}", x),
            Value::Ref(x, y) => write!(f, "({},{})", x, y),
            Value::Empty => write!(f, ""),
            _ => write!(f, "Error"),
        }
    }
}