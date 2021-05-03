use super::*;

impl Table {
    /// Expand a CSVX table vertically
    ///
    /// Add a row to the bottom end.
    ///
    /// # Example
    /// ```
    /// table.extend_y();
    /// println!("TABLE:\n{}", table);
    /// ```
    pub fn extend_y(&mut self) {
        let raw_table = &mut self.raw_table;
        let tree_table = &mut self.tree_table;
        let refs_table = &mut self.refs_table;
        let refs_to_table = &mut self.refs_to_table;
        let calculated_table = &mut self.calculated_table;

        raw_table.push(if let Some(last) = raw_table.last() {
            (0..last.len()).map(|_| String::new()).collect()
        } else {
            vec![]
        });

        tree_table.push(if let Some(last) = tree_table.last() {
            (0..last.len())
                .map(|_| Box::new(Value::Empty) as Box<dyn Node>)
                .collect()
        } else {
            vec![]
        });

        refs_table.push(if let Some(last) = refs_table.last() {
            (0..last.len()).map(|_| vec![]).collect()
        } else {
            vec![]
        });

        refs_to_table.push(if let Some(last) = refs_to_table.last() {
            (0..last.len()).map(|_| vec![]).collect()
        } else {
            vec![]
        });

        calculated_table.push(if let Some(last) = calculated_table.last() {
            (0..last.len()).map(|_| Value::Empty).collect()
        } else {
            vec![]
        });
    }

    /// Expand a CSVX table horizontally
    ///
    /// Add a column to the right end.
    ///
    /// # Example
    /// ```
    /// table.extend_x();
    /// println!("TABLE:\n{}", table);
    /// ```
    pub fn extend_x(&mut self) {
        let raw_table = &mut self.raw_table;
        let tree_table = &mut self.tree_table;
        let refs_table = &mut self.refs_table;
        let refs_to_table = &mut self.refs_to_table;
        let calculated_table = &mut self.calculated_table;

        for line in raw_table {
            line.push(String::new());
        }
        for line in tree_table {
            line.push(Box::new(Value::Empty));
        }
        for line in refs_table {
            line.push(vec![]);
        }
        for line in refs_to_table {
            line.push(vec![]);
        }
        for line in calculated_table {
            line.push(Value::Empty);
        }
    }
}
