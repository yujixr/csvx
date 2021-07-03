use super::*;

pub struct Node {
    leaf: Box<ThreadSafeNode>,
}

impl super::Node for Node {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>) {
        let (leaf, refs) = parse(&seqs[0]);
        (Box::new(Self { leaf }), refs)
    }
    fn calc(
        &mut self,
        calculated_table: &Vec<Vec<Value>>,
    ) -> (Value, Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let leaf = self.leaf.calc(calculated_table);

        let value = match leaf.0 {
            Value::Integer(leaf) => Value::Integer(-leaf),
            Value::Float(leaf) => Value::Float(-leaf),
            _ => Value::Error,
        };

        (value, leaf.1, leaf.2)
    }
}
