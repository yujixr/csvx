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
        let ref_src_table = &mut self.ref_src_table;
        let dependents_table = &mut self.dependents_table;
        let calculated_table = &mut self.calculated_table;

        if raw_table.len() == 1 {
            return;
        }

        raw_table.remove(y);
        tree_table.remove(y);
        dependents_table.remove(y);
        calculated_table.remove(y);

        *ref_src_table = ref_src_table
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

        for y_of_src in y..ref_src_table.len() {
            for x_of_src in 0..ref_src_table[y_of_src].len() {
                for i in 0..ref_src_table[y_of_src][x_of_src].len() {
                    let (x, y) = ref_src_table[y_of_src][x_of_src][i];
                    Self::calc(
                        x,
                        y,
                        tree_table,
                        ref_src_table,
                        dependents_table,
                        calculated_table,
                        &mut vec![],
                    );
                }
            }
        }

        ref_src_table.swap_remove(ref_src_table.len() - 1);
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
        let ref_src_table = &mut self.ref_src_table;
        let dependents_table = &mut self.dependents_table;
        let calculated_table = &mut self.calculated_table;

        if let Some(row) = raw_table.get(0) {
            if row.len() == 1 {
                return;
            }
        } else {
            return;
        }

        for line in raw_table {
            line.remove(x);
        }
        for line in tree_table.iter_mut() {
            line.remove(x);
        }
        for line in dependents_table.iter_mut() {
            line.remove(x);
        }
        for line in calculated_table.iter_mut() {
            line.remove(x);
        }

        *ref_src_table = ref_src_table
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

        for y_of_src in 0..ref_src_table.len() {
            for x_of_src in x..ref_src_table[y_of_src].len() {
                for i in 0..ref_src_table[y_of_src][x_of_src].len() {
                    let (x, y) = ref_src_table[y_of_src][x_of_src][i];
                    Self::calc(
                        x,
                        y,
                        tree_table,
                        ref_src_table,
                        dependents_table,
                        calculated_table,
                        &mut vec![],
                    );
                }
            }
        }

        for line in ref_src_table {
            line.swap_remove(line.len() - 1);
        }
    }
}
