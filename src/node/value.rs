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
    Range(usize, usize, usize, usize),
    Empty,
}

impl Node for Value {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>) {
        let val = seqs[0][0].clone();
        match val {
            Token::Integer(x) => (Box::new(Value::Integer(x)), vec![]),
            Token::Float(x) => (Box::new(Value::Float(x)), vec![]),
            Token::String(x) => (Box::new(Value::String(x)), vec![]),
            Token::Boolean(x) => (Box::new(Value::Boolean(x)), vec![]),
            Token::Ref(x, y) => (Box::new(Value::Ref(x, y)), vec![(x, y)]),
            Token::Range(x1, y1, x2, y2) => (Box::new(Value::Range(x1, y1, x2, y2)), {
                let rx = x1.min(x2)..=x1.max(x2);
                let ry = y1.min(y2)..=y1.max(y2);
                rx.flat_map(|x| {
                    return ry.clone().map(move |y| (x, y));
                })
                .collect()
            }),
            _ => (Box::new(Value::Error), vec![]),
        }
    }
    fn calc(&self, calculated_table: &Vec<Vec<Value>>) -> Value {
        match self {
            Value::Ref(x, y) => {
                if let Some(row) = calculated_table.get(*y) {
                    if let Some(item) = row.get(*x) {
                        item.clone()
                    } else {
                        Value::Error
                    }
                } else {
                    Value::Error
                }
            }
            _ => self.clone(),
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
            Value::Range(x1, y1, x2, y2) => write!(f, "({},{}) : ({},{})", x1, y1, x2, y2),
            Value::Empty => write!(f, ""),
            _ => write!(f, "Error"),
        }
    }
}
