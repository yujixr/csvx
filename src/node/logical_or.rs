use super::*;

pub struct LogicalOr {
    left: Box<dyn Node>,
    right: Box<dyn Node>,
}

impl Node for LogicalOr {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<dyn Node>, Vec<(usize, usize)>) {
        let (left, mut left_refs) = parse(&seqs[0]);
        let (right, mut right_refs) = parse(&seqs[1]);
        left_refs.append(&mut right_refs);
        (Box::new(Self { left, right }), left_refs)
    }
    fn calc(&self, calculated_table: &Vec<Vec<Value>>) -> Value {
        let left = self.left.calc(calculated_table);
        let right = self.right.calc(calculated_table);
        match (left, right) {
            (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left || right),
            _ => Value::Error,
        }
    }
}
