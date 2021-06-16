use super::*;

pub struct Node {
    leaf: Box<ThreadSafeNode>,
}

impl super::Node for Node {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>) {
        let (leaf, refs) = parse(&seqs[0]);
        (Box::new(Self { leaf }), refs)
    }
    fn calc(&self, calculated_table: &Vec<Vec<Value>>) -> Value {
        let leaf = self.leaf.calc(calculated_table);
        match leaf {
            Value::Integer(leaf) => Value::Integer(-leaf),
            Value::Float(leaf) => Value::Float(-leaf),
            _ => Value::Error,
        }
    }
}
