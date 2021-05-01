use super::*;

pub struct FnLog10 {
    leaf: Box<dyn Node>,
}

impl Node for FnLog10 {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<dyn Node>, Vec<(usize, usize)>) {
        let (leaf, leaf_refs) = parse(&seqs[0]);
        (Box::new(Self { leaf }), leaf_refs)
    }
    fn calc(&self, calculated_table: &Vec<Vec<Value>>) -> Value {
        let leaf = self.leaf.calc(calculated_table);
        match leaf {
            Value::Integer(x) => Value::Float((x as f64).log10()),
            Value::Float(x) => Value::Float(x.log10()),
            _ => Value::Error,
        }
    }
}
