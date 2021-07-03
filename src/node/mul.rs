use super::*;

pub struct Node {
    left: Box<ThreadSafeNode>,
    right: Box<ThreadSafeNode>,
}

impl super::Node for Node {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>) {
        let (left, mut left_refs) = parse(&seqs[0]);
        let (right, mut right_refs) = parse(&seqs[1]);
        left_refs.append(&mut right_refs);
        (Box::new(Self { left, right }), left_refs)
    }
    fn calc(
        &mut self,
        calculated_table: &Vec<Vec<Value>>,
    ) -> (Value, Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let mut left = self.left.calc(calculated_table);
        let mut right = self.right.calc(calculated_table);

        let value = match (left.0, right.0) {
            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left * right),
            (Value::Integer(left), Value::Float(right)) => Value::Float(left as f64 * right),
            (Value::Float(left), Value::Integer(right)) => Value::Float(left * right as f64),
            (Value::Float(left), Value::Float(right)) => Value::Float(left * right),
            _ => Value::Error,
        };

        left.1.append(&mut right.1);
        left.2.append(&mut right.2);
        (value, left.1, left.2)
    }
}
