use super::*;

pub struct FnTan {
    leaf: Box<ThreadSafeNode>,
}

impl Node for FnTan {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>) {
        let (leaf, leaf_refs) = parse(&seqs[0]);
        (Box::new(Self { leaf }), leaf_refs)
    }
    fn calc(&self, calculated_table: &Vec<Vec<Value>>) -> Value {
        let leaf = self.leaf.calc(calculated_table);
        match leaf {
            Value::Integer(x) => Value::Float((x as f64).tan()),
            Value::Float(x) => Value::Float(x.tan()),
            _ => Value::Error,
        }
    }
}
