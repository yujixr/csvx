use super::*;

impl Table {
    /// Remove a row at position y.
    ///
    /// # Example
    /// ```
    /// table.remove_y(2);
    /// println!("TABLE:\n{}", table);
    /// ```
    pub fn remove_y(&mut self, y: usize) {
        let raw_table = &mut self.raw_table;
        let tree_table = &mut self.tree_table;
        let refs_table = &mut self.refs_table;
        let refs_to_table = &mut self.refs_to_table;
        let calculated_table = &mut self.calculated_table;

        raw_table.remove(y);
        tree_table.remove(y);
        refs_to_table.remove(y);
        calculated_table.remove(y);

        *refs_table = refs_table
            .iter()
            .map(|refs_row| {
                refs_row
                    .iter()
                    .map(|refs| {
                        refs.iter()
                            .filter_map(|&(x_of_target, y_of_target)| {
                                if y_of_target == y {
                                    None
                                } else if y_of_target > y {
                                    Some((x_of_target, y_of_target - 1))
                                } else {
                                    Some((x_of_target, y_of_target))
                                }
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();

        for y_of_src in y..refs_table.len() {
            for x_of_src in 0..refs_table[y_of_src].len() {
                for &(x, y) in &refs_table[y_of_src][x_of_src] {
                    if x != x_of_src || y != y_of_src {
                        Self::calc(x, y, tree_table, refs_table, calculated_table);
                    }
                }
            }
        }

        refs_table.swap_remove(refs_table.len() - 1);
    }

    /// Remove a column at position x.
    ///
    /// # Example
    /// ```
    /// table.remove_x(2);
    /// println!("TABLE:\n{}", table);
    /// ```
    pub fn remove_x(&mut self, x: usize) {
        let raw_table = &mut self.raw_table;
        let tree_table = &mut self.tree_table;
        let refs_table = &mut self.refs_table;
        let refs_to_table = &mut self.refs_to_table;
        let calculated_table = &mut self.calculated_table;

        for line in raw_table {
            line.remove(x);
        }
        for line in tree_table.iter_mut() {
            line.remove(x);
        }
        for line in refs_to_table {
            line.remove(x);
        }
        for line in calculated_table.iter_mut() {
            line.remove(x);
        }

        *refs_table = refs_table
            .iter()
            .map(|refs_row| {
                refs_row
                    .iter()
                    .map(|refs| {
                        refs.iter()
                            .filter_map(|&(x_of_target, y_of_target)| {
                                if x_of_target == x {
                                    None
                                } else if x_of_target > x {
                                    Some((x_of_target - 1, y_of_target))
                                } else {
                                    Some((x_of_target, y_of_target))
                                }
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();

        for y_of_src in 0..refs_table.len() {
            for x_of_src in x..refs_table[y_of_src].len() {
                for &(x, y) in &refs_table[y_of_src][x_of_src] {
                    if x != x_of_src || y != y_of_src {
                        Self::calc(x, y, tree_table, refs_table, calculated_table);
                    }
                }
            }
        }

        for line in refs_table {
            line.swap_remove(line.len() - 1);
        }
    }
}
