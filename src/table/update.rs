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
        let ref_src_table = &mut self.ref_src_table;
        let dependents_table = &mut self.dependents_table;
        let calculated_table = &mut self.calculated_table;

        if raw_table.len() <= y || raw_table[y].len() <= x {
            return Err(TableError::OutOfRange { x, y });
        }
        raw_table[y][x] = raw_string.borrow().to_string();

        for (x_of_src, y_of_src) in dependents_table[y][x].keys() {
            let x_of_src = *x_of_src;
            let y_of_src = *y_of_src;
            if y_of_src < ref_src_table.len() && x_of_src < ref_src_table[y].len() {
                ref_src_table[y_of_src][x_of_src].retain(|&v| v != (x, y));
            }
        }

        let (tree, refs) = Self::build_tree(&raw_string);

        let mut dependents = HashMap::new();
        for dependent in refs {
            *(dependents.entry(dependent).or_insert(0)) += 1;
        }

        tree_table[y][x] = tree;
        dependents_table[y][x] = dependents;

        for &(x_of_src, y_of_src) in dependents_table[y][x].keys() {
            if y_of_src < ref_src_table.len() && x_of_src < ref_src_table[y].len() {
                ref_src_table[y_of_src][x_of_src].push((x, y));
            }
        }

        Self::calc(
            x,
            y,
            tree_table,
            ref_src_table,
            dependents_table,
            calculated_table,
            &mut vec![],
        );
        Ok(())
    }
}
