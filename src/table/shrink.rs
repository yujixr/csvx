use super::*;

impl Table {
    /// Shrink a CSVX table vertically
    ///
    /// Delete a row at the bottom end.
    ///
    /// # Example
    /// ```
    /// table.shrink_y();
    /// println!("TABLE:\n{}", table);
    /// ```
    pub fn shrink_y(&mut self) {
        let raw_table = &mut self.raw_table;
        let tree_table = &mut self.tree_table;
        let refs_table = &mut self.refs_table;
        let calculated_table = &mut self.calculated_table;

        calculated_table.swap_remove(calculated_table.len() - 1);

        let y_last = refs_table.len() - 1;
        for y in 0..refs_table.len() {
            for x in 0..refs_table[y].len() {
                refs_table[y][x].retain(|&v| v.1 != y_last);
            }
        }

        if let Some(refs) = refs_table.last() {
            for refs in refs {
                for (x, y) in refs {
                    Self::calc(*x, *y, tree_table, refs_table, calculated_table);
                }
            }
        }

        raw_table.swap_remove(raw_table.len() - 1);
        tree_table.swap_remove(tree_table.len() - 1);
        refs_table.swap_remove(refs_table.len() - 1);
    }

    /// Shrink a CSVX table horizontally
    ///
    /// Delete a column at the right end.
    ///
    /// # Example
    /// ```
    /// table.shrink_x();
    /// println!("TABLE:\n{}", table);
    /// ```
    pub fn shrink_x(&mut self) {
        let raw_table = &mut self.raw_table;
        let tree_table = &mut self.tree_table;
        let refs_table = &mut self.refs_table;
        let calculated_table = &mut self.calculated_table;

        if let Some(line) = refs_table.first() {
            let x_last = line.len() - 1;
            for y in 0..refs_table.len() {
                calculated_table[y].swap_remove(x_last);
                for x in 0..refs_table[y].len() {
                    refs_table[y][x].retain(|&v| v.0 != x_last);
                }
            }
        }

        for y_of_src in 0..refs_table.len() {
            if let Some(refs) = refs_table[y_of_src].last() {
                for (x, y) in refs {
                    Self::calc(*x, *y, tree_table, refs_table, calculated_table);
                }
            }
        }

        for line in raw_table {
            line.swap_remove(line.len() - 1);
        }
        for line in tree_table {
            line.swap_remove(line.len() - 1);
        }
        for line in refs_table {
            line.swap_remove(line.len() - 1);
        }
    }
}
