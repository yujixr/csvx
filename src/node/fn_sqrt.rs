use super::*;

pub struct Node {
    leaf: Box<ThreadSafeNode>,
}

impl super::Node for Node {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>) {
        let (leaf, leaf_refs) = parse(&seqs[0]);
        (Box::new(Self { leaf }), leaf_refs)
    }
    fn calc(
        &mut self,
        calculated_table: &Vec<Vec<Value>>,
    ) -> (Value, Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let leaf = self.leaf.calc(calculated_table);

        let value = match leaf.0 {
            Value::Integer(x) => Value::Float((x as f64).sqrt()),
            Value::Float(x) => Value::Float(x.sqrt()),
            _ => Value::Error,
        };

        (value, leaf.1, leaf.2)
    }
}
