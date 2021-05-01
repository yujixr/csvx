use super::*;

pub struct FnIf {
    condition: Box<dyn Node>,
    on_true: Box<dyn Node>,
    on_false: Box<dyn Node>,
}

impl Node for FnIf {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<dyn Node>, Vec<(usize, usize)>) {
        let (condition, mut condition_refs) = parse(&seqs[0]);
        let (on_true, mut on_true_refs) = parse(&seqs[1]);
        let (on_false, mut on_false_refs) = parse(&seqs[2]);
        condition_refs.append(&mut on_true_refs);
        condition_refs.append(&mut on_false_refs);
        (
            Box::new(Self {
                condition,
                on_true,
                on_false,
            }),
            condition_refs,
        )
    }
    fn calc(&self, calculated_table: &Vec<Vec<Value>>) -> Value {
        let condition = self.condition.calc(calculated_table);
        let on_true = self.on_true.calc(calculated_table);
        let on_false = self.on_false.calc(calculated_table);
        if let Value::Boolean(condition) = condition {
            if condition {
                on_true
            } else {
                on_false
            }
        } else {
            Value::Error
        }
    }
}
