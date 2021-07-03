use super::*;

pub struct Node {
    condition: Box<ThreadSafeNode>,
    on_true: Box<ThreadSafeNode>,
    on_false: Box<ThreadSafeNode>,
}

impl super::Node for Node {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>) {
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
    fn calc(
        &mut self,
        calculated_table: &Vec<Vec<Value>>,
    ) -> (Value, Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let mut condition = self.condition.calc(calculated_table);
        let mut on_true = self.on_true.calc(calculated_table);
        let mut on_false = self.on_false.calc(calculated_table);

        let value = if let Value::Boolean(condition) = condition.0 {
            if condition {
                on_true.0
            } else {
                on_false.0
            }
        } else {
            Value::Error
        };

        condition.1.append(&mut on_true.1);
        condition.1.append(&mut on_false.1);
        condition.2.append(&mut on_true.2);
        condition.2.append(&mut on_false.2);
        (value, condition.1, condition.2)
    }
}
