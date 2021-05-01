use super::*;

pub struct FnLog {
    base: Box<dyn Node>,
    number: Box<dyn Node>,
}

impl Node for FnLog {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<dyn Node>, Vec<(usize, usize)>) {
        let (base, mut base_refs) = parse(&seqs[0]);
        let (number, mut number_refs) = parse(&seqs[1]);
        base_refs.append(&mut number_refs);
        (Box::new(Self { base, number }), base_refs)
    }
    fn calc(&self, calculated_table: &Vec<Vec<Value>>) -> Value {
        let base = self.base.calc(calculated_table);
        let number = self.number.calc(calculated_table);
        match (base, number) {
            (Value::Integer(base), Value::Integer(number)) => {
                Value::Float((number as f64).log(base as f64))
            }
            (Value::Integer(base), Value::Float(number)) => Value::Float(number.log(base as f64)),
            (Value::Float(base), Value::Integer(number)) => Value::Float((number as f64).log(base)),
            (Value::Float(base), Value::Float(number)) => Value::Float(number.log(base)),
            _ => Value::Error,
        }
    }
}
