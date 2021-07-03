use super::*;

pub struct Node {
    base: Box<ThreadSafeNode>,
    exp: Box<ThreadSafeNode>,
}

impl super::Node for Node {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>) {
        let (base, mut base_refs) = parse(&seqs[0]);
        let (exp, mut exp_refs) = parse(&seqs[1]);
        base_refs.append(&mut exp_refs);
        (Box::new(Self { base, exp }), base_refs)
    }
    fn calc(
        &mut self,
        calculated_table: &Vec<Vec<Value>>,
    ) -> (Value, Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let mut base = self.base.calc(calculated_table);
        let mut exp = self.exp.calc(calculated_table);

        let value = match (base.0, exp.0) {
            (Value::Integer(base), Value::Integer(exp)) => {
                if exp >= 0 {
                    Value::Integer(base.pow(exp as u32))
                } else {
                    Value::Float((base as f64).powi(exp as i32))
                }
            }
            (Value::Integer(base), Value::Float(exp)) => Value::Float((base as f64).powf(exp)),
            (Value::Float(base), Value::Integer(exp)) => Value::Float(base.powi(exp as i32)),
            (Value::Float(base), Value::Float(exp)) => Value::Float(base.powf(exp)),
            _ => Value::Error,
        };

        base.1.append(&mut exp.1);
        base.2.append(&mut exp.2);
        (value, base.1, base.2)
    }
}
