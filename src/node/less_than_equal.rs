use super::*;

pub struct LessThanEqual {
    left: Box<ThreadSafeNode>,
    right: Box<ThreadSafeNode>,
}

impl Node for LessThanEqual {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>) {
        let (left, mut left_refs) = parse(&seqs[0]);
        let (right, mut right_refs) = parse(&seqs[1]);
        left_refs.append(&mut right_refs);
        (Box::new(Self { left, right }), left_refs)
    }
    fn calc(&self, calculated_table: &Vec<Vec<Value>>) -> Value {
        let left = self.left.calc(calculated_table);
        let right = self.right.calc(calculated_table);
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left <= right),
            (Value::Integer(left), Value::Float(right)) => Value::Boolean(left as f64 <= right),
            (Value::Float(left), Value::Integer(right)) => Value::Boolean(left <= right as f64),
            (Value::Float(left), Value::Float(right)) => Value::Boolean(left <= right),
            (Value::String(left), Value::String(right)) => Value::Boolean(left <= right),
            (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left <= right),
            _ => Value::Error,
        }
    }
}
