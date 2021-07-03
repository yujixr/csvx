use super::*;

pub struct Node {
    x: Box<ThreadSafeNode>,
    y: Box<ThreadSafeNode>,
    old_dependent: Option<(usize, usize)>,
}

impl super::Node for Node {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>) {
        let (x, mut x_refs) = parse(&seqs[0]);
        let (y, mut y_refs) = parse(&seqs[1]);
        x_refs.append(&mut y_refs);
        (
            Box::new(Self {
                x,
                y,
                old_dependent: None,
            }),
            x_refs,
        )
    }
    fn calc(
        &mut self,
        calculated_table: &Vec<Vec<Value>>,
    ) -> (Value, Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let mut x = self.x.calc(calculated_table);
        let mut y = self.y.calc(calculated_table);

        if let Some(old_dependent) = self.old_dependent {
            x.1.push(old_dependent);
        }
        x.1.append(&mut y.1);
        x.2.append(&mut y.2);

        let value = match (x.0, y.0) {
            (Value::Integer(depemndent_x), Value::Integer(depemndent_y)) => {
                if 0 <= depemndent_y
                    && (depemndent_y as usize) < calculated_table.len()
                    && 0 <= depemndent_x
                    && (depemndent_x as usize) < calculated_table[depemndent_y as usize].len()
                {
                    let depemndent_x = depemndent_x as usize;
                    let depemndent_y = depemndent_y as usize;

                    x.2.push((depemndent_x, depemndent_y));
                    self.old_dependent = Some((depemndent_x, depemndent_y));

                    calculated_table[depemndent_y][depemndent_x].clone()
                } else {
                    self.old_dependent = None;
                    Value::Error
                }
            }
            _ => {
                self.old_dependent = None;
                Value::Error
            }
        };

        (value, x.1, x.2)
    }
}
