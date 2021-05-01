use super::*;

pub struct FnPow {
    base: Box<dyn Node>,
    exp: Box<dyn Node>,
}

impl Node for FnPow {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<dyn Node>, Vec<(usize, usize)>) {
        let (base, mut base_refs) = parse(&seqs[0]);
        let (exp, mut exp_refs) = parse(&seqs[1]);
        base_refs.append(&mut exp_refs);
        (Box::new(Self { base, exp }), base_refs)
    }
    fn calc(&self, calculated_table: &Vec<Vec<Value>>) -> Value {
        let base = self.base.calc(calculated_table);
        let exp = self.exp.calc(calculated_table);
        match (base, exp) {
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
        }
    }
}