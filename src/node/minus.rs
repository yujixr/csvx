use super::*;

pub struct Minus {
    leaf: Box<dyn Node>,
}

impl Node for Minus {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<dyn Node>, Vec<(usize, usize)>) {
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
