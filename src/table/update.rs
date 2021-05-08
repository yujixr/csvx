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
        let refs_to_table = &mut self.refs_to_table;
        let calculated_table = &mut self.calculated_table;

        if raw_table.len() <= y || raw_table[y].len() <= x {
            return Err(TableError::OutOfRange { x, y });
        }
        raw_table[y][x] = raw_string.borrow().to_string();

        for (x_of_src, y_of_src) in &refs_to_table[y][x] {
            let x_of_src = *x_of_src;
            let y_of_src = *y_of_src;
            if y_of_src < refs_table.len() && x_of_src < refs_table[y].len() {
                refs_table[y_of_src][x_of_src].retain(|&v| v != (x, y));
            }
        }

        let (tree, refs) = Self::build_tree(raw_string.borrow().to_string());
        tree_table[y][x] = tree;
        refs_to_table[y][x] = refs;

        for &(x_of_src, y_of_src) in &refs_to_table[y][x] {
            if y_of_src < refs_table.len() && x_of_src < refs_table[y].len() {
                refs_table[y_of_src][x_of_src].push((x, y));
            }
        }

        if !refs_table[y][x].contains(&(x, y)) {
            Self::calc(x, y, &tree_table, &refs_table, calculated_table);
        }
        Ok(())
    }
}
