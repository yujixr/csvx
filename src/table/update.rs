use super::*;

impl Table {
    /// Update an item within a CSVX table
    ///
    /// use 0-indexed values for `x` and `y`.
    ///
    /// # Example
    /// ```
    /// table.update(0, 0, "true")?;
    /// println!("UPDATED TABLE:\n{}", table.export_calculated_table()?);
    /// ```
    pub fn update<T: Borrow<str>>(
        &mut self,
        x: usize,
        y: usize,
        raw_string: T,
    ) -> Result<(), TableError> {
        let raw_table = &mut self.raw_table;
        let tree_table = &mut self.tree_table;
        let refs_table = &mut self.refs_table;
        let calculated_table = &mut self.calculated_table;

        if raw_table.len() <= y || raw_table[y].len() <= x {
            return Err(TableError::OutOfRange { x, y });
        }
        raw_table[y][x] = raw_string.borrow().to_string();

        for y_of_src in 0..refs_table.len() {
            for x_or_src in 0..refs_table[y_of_src].len() {
                refs_table[y_of_src][x_or_src].retain(|&v| v != (x, y));
            }
        }

        let (tree, refs) = Self::build_tree(raw_string.borrow().to_string());
        tree_table[y][x] = tree;

        for (x_of_src, y_of_src) in refs {
            refs_table[y_of_src as usize][x_of_src as usize].push((x, y));
        }

        Self::calc(x, y, &tree_table, &refs_table, calculated_table);
        Ok(())
    }
}
