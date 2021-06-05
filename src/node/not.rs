use super::*;

pub struct Not {
    leaf: Box<ThreadSafeNode>,
}

impl Node for Not {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>) {
        let (leaf, refs) = parse(&seqs[0]);
        (Box::new(Self { leaf }), refs)
    }
    fn calc(&self, calculated_table: &Vec<Vec<Value>>) -> Value {
        let leaf = self.leaf.calc(calculated_table);
        match leaf {
            Value::Integer(leaf) => Value::Integer(!leaf),
            Value::Boolean(leaf) => Value::Boolean(!leaf),
            _ => Value::Error,
        }
    }
}
