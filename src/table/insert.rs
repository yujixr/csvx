use super::*;

impl Table {
    /// Insert a row at position y.
    ///
    /// # Example
    /// ```
    /// table.insert_y(2);
    /// println!("TABLE:\n{}", table);
    /// ```
    pub fn insert_y(&mut self, y: usize) {
        let raw_table = &mut self.raw_table;
        let tree_table = &mut self.tree_table;
        let refs_table = &mut self.refs_table;
        let refs_to_table = &mut self.refs_to_table;
        let calculated_table = &mut self.calculated_table;

        raw_table.insert(
            y,
            if let Some(last) = raw_table.last() {
                (0..last.len()).map(|_| String::new()).collect()
            } else {
                vec![]
            },
        );

        tree_table.insert(
            y,
            if let Some(last) = tree_table.last() {
                (0..last.len())
                    .map(|_| Box::new(Value::Empty) as Box<ThreadSafeNode>)
                    .collect()
            } else {
                vec![]
            },
        );

        refs_table.push(if let Some(last) = refs_table.last() {
            (0..last.len()).map(|_| vec![]).collect()
        } else {
            vec![]
        });

        refs_to_table.insert(
            y,
            if let Some(last) = refs_to_table.last() {
                (0..last.len()).map(|_| vec![]).collect()
            } else {
                vec![]
            },
        );

        calculated_table.insert(
            y,
            if let Some(last) = calculated_table.last() {
                (0..last.len()).map(|_| Value::Empty).collect()
            } else {
                vec![]
            },
        );

        *refs_table = refs_table
            .iter()
            .map(|refs_row| {
                refs_row
                    .iter()
                    .map(|refs| {
                        refs.iter()
                            .map(|&(x_of_target, y_of_target)| {
                                if y_of_target >= y {
                                    (x_of_target, y_of_target + 1)
                                } else {
                                    (x_of_target, y_of_target)
                                }
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();

        for y_of_src in (y + 1)..refs_table.len() {
            for x_of_src in 0..refs_table[y_of_src].len() {
                for &(x, y) in &refs_table[y_of_src][x_of_src] {
                    if x != x_of_src || y != y_of_src {
                        Self::calc(x, y, tree_table, refs_table, calculated_table);
                    }
                }
            }
        }
    }

    /// Insert a column at position x.
    ///
    /// # Example
    /// ```
    /// table.insert_x(2);
    /// println!("TABLE:\n{}", table);
    /// ```
    pub fn insert_x(&mut self, x: usize) {
        let raw_table = &mut self.raw_table;
        let tree_table = &mut self.tree_table;
        let refs_table = &mut self.refs_table;
        let refs_to_table = &mut self.refs_to_table;
        let calculated_table = &mut self.calculated_table;

        for line in raw_table.iter_mut() {
            line.insert(x, String::new());
        }
        for line in tree_table.iter_mut() {
            line.insert(x, Box::new(Value::Empty));
        }
        for line in refs_table.iter_mut() {
            line.push(vec![]);
        }
        for line in refs_to_table.iter_mut() {
            line.insert(x, vec![]);
        }
        for line in calculated_table.iter_mut() {
            line.insert(x, Value::Empty);
        }

        *refs_table = refs_table
            .iter()
            .map(|refs_row| {
                refs_row
                    .iter()
                    .map(|refs| {
                        refs.iter()
                            .map(|&(x_of_target, y_of_target)| {
                                if x_of_target >= x {
                                    (x_of_target + 1, y_of_target)
                                } else {
                                    (x_of_target, y_of_target)
                                }
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();

        for y_of_src in 0..refs_table.len() {
            for x_of_src in (x + 1)..refs_table[y_of_src].len() {
                for &(x, y) in &refs_table[y_of_src][x_of_src] {
                    if x != x_of_src || y != y_of_src {
                        Self::calc(x, y, tree_table, refs_table, calculated_table);
                    }
                }
            }
        }
    }
}
