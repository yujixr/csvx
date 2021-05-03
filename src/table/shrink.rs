use super::*;

impl Table {
    /// Shrink a CSVX table vertically
    ///
    /// Delete a row at the bottom end.
    ///
    /// # Example
    /// ```
    /// table.shrink_y();
    /// println!("RAW TABLE:\n{}", table.export_raw_table()?);
    /// ```
    pub fn shrink_y(&mut self) {
        let raw_table = &mut self.raw_table;
        let tree_table = &mut self.tree_table;
        let refs_table = &mut self.refs_table;
        let calculated_table = &mut self.calculated_table;

        raw_table.swap_remove(raw_table.len() - 1);
        tree_table.swap_remove(tree_table.len() - 1);
        refs_table.swap_remove(refs_table.len() - 1);
        calculated_table.swap_remove(calculated_table.len() - 1);
    }

    /// Shrink a CSVX table horizontally
    ///
    /// Delete a column at the right end.
    ///
    /// # Example
    /// ```
    /// table.shrink_x();
    /// println!("RAW TABLE:\n{}", table.export_raw_table()?);
    /// ```
    pub fn shrink_x(&mut self) {
        let raw_table = &mut self.raw_table;
        let tree_table = &mut self.tree_table;
        let refs_table = &mut self.refs_table;
        let calculated_table = &mut self.calculated_table;

        for line in raw_table {
            line.swap_remove(line.len() - 1);
        }
        for line in tree_table {
            line.swap_remove(line.len() - 1);
        }
        for line in refs_table {
            line.swap_remove(line.len() - 1);
        }
        for line in calculated_table {
            line.swap_remove(line.len() - 1);
        }
    }
}
