use super::*;

pub struct Node {
    base: Box<ThreadSafeNode>,
    number: Box<ThreadSafeNode>,
}

impl super::Node for Node {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>) {
        let (base, mut base_refs) = parse(&seqs[0]);
        let (number, mut number_refs) = parse(&seqs[1]);
        base_refs.append(&mut number_refs);
        (Box::new(Self { base, number }), base_refs)
    }
    fn calc(
        &mut self,
        calculated_table: &Vec<Vec<Value>>,
    ) -> (Value, Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let mut base = self.base.calc(calculated_table);
        let mut number = self.number.calc(calculated_table);

        let value = match (base.0, number.0) {
            (Value::Integer(base), Value::Integer(number)) => {
                Value::Float((number as f64).log(base as f64))
            }
            (Value::Integer(base), Value::Float(number)) => Value::Float(number.log(base as f64)),
            (Value::Float(base), Value::Integer(number)) => Value::Float((number as f64).log(base)),
            (Value::Float(base), Value::Float(number)) => Value::Float(number.log(base)),
            _ => Value::Error,
        };

        base.1.append(&mut number.1);
        base.2.append(&mut number.2);
        (value, base.1, base.2)
    }
}
