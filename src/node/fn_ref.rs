use super::*;

pub struct FnRef {
    x: Box<dyn Node>,
    y: Box<dyn Node>,
}

impl Node for FnRef {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<dyn Node>, Vec<(usize, usize)>) {
        let (x, mut x_refs) = parse(&seqs[0]);
        let (y, mut y_refs) = parse(&seqs[1]);
        x_refs.append(&mut y_refs);
        (Box::new(Self { x, y }), x_refs)
    }
    fn calc(&self, calculated_table: &Vec<Vec<Value>>) -> Value {
        let x = self.x.calc(calculated_table);
        let y = self.y.calc(calculated_table);
        match (x, y) {
            (Value::Integer(x), Value::Integer(y)) => {
                if 0 <= y
                    && (y as usize) < calculated_table.len()
                    && 0 <= x
                    && (x as usize) < calculated_table[y as usize].len()
                {
                    calculated_table[y as usize][x as usize].clone()
                } else {
                    Value::Error
                }
            }
            _ => Value::Error,
        }
    }
}
