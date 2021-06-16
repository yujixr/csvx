use super::*;

pub struct Node {
    leaf: Box<ThreadSafeNode>,
}

impl super::Node for Node {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>) {
        let (leaf, leaf_refs) = parse(&seqs[0]);
        (Box::new(Self { leaf }), leaf_refs)
    }
    fn calc(&self, calculated_table: &Vec<Vec<Value>>) -> Value {
        let leaf = self.leaf.calc(calculated_table);
        match leaf {
            Value::Range(x1, y1, x2, y2) => {
                let rx: Vec<usize> = (x1.min(x2)..=x1.max(x2)).collect();
                let ry: Vec<usize> = (y1.min(y2)..=y1.max(y2)).collect();

                ry.iter().fold(Value::Integer(0), |b, y| {
                    if let Some(row) = calculated_table.get(*y) {
                        let sum_of_row =
                            rx.iter()
                                .fold(Value::Integer(0), |b, x| match (b, row.get(*x)) {
                                    (Value::Integer(b), Some(Value::Integer(item))) => {
                                        Value::Integer(b + item)
                                    }
                                    (Value::Integer(b), Some(Value::Float(item))) => {
                                        Value::Float(b as f64 + item)
                                    }
                                    (Value::Float(b), Some(Value::Integer(item))) => {
                                        Value::Float(b + *item as f64)
                                    }
                                    (Value::Float(b), Some(Value::Float(item))) => {
                                        Value::Float(b + item)
                                    }
                                    _ => Value::Error,
                                });

                        match (b, sum_of_row) {
                            (Value::Integer(b), Value::Integer(row)) => Value::Integer(b + row),
                            (Value::Integer(b), Value::Float(row)) => Value::Float(b as f64 + row),
                            (Value::Float(b), Value::Integer(row)) => Value::Float(b + row as f64),
                            (Value::Float(b), Value::Float(row)) => Value::Float(b + row),
                            _ => Value::Error,
                        }
                    } else {
                        Value::Error
                    }
                })
            }
            _ => Value::Error,
        }
    }
}
