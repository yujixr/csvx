use super::*;

impl Table {
    /// Build a CSVX table from a CSV string.
    ///
    /// # Example
    /// ```
    /// use csvx::Table;
    ///
    /// let raw_csv = "pi,3^5,\"ref(0,0)\",-(1/0)
    /// 12%5,\"pow(3,5)\",0/NaN,\"\"\"Apollo\"\"\"
    /// A1+A2,\"if(true , sqrt(25),round(if(false,1.1,2.5)))\",D2+1969,";
    /// println!("RAW CSV DATA:\n{}\n", raw_csv);
    ///
    /// let mut table = Table::new(raw_csv)?;
    /// println!("CALCULATED TABLE:\n{}", table);
    /// ```
    pub fn new<T: Borrow<str>>(raw_csv: T) -> Result<Self, Box<dyn Error>> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(raw_csv.borrow().as_bytes());
        let mut raw_table: Vec<Vec<String>> = vec![];

        for record in reader.records() {
            match record {
                Ok(record) => raw_table.push(record.iter().map(|item| item.to_owned()).collect()),
                _ => return Err(Box::new(TableError::CSVParseError)),
            }
        }

        let mut raw_string_table = vec![];
        let mut tree_table = vec![];
        let mut refs_table = vec![];
        let mut refs_to_table = vec![];
        let mut calculated_table = vec![];
        for raw_line in raw_table {
            let mut raw_string_line = vec![];
            let mut tree_line = vec![];
            let mut refs_line = vec![];
            let mut refs_to_line = vec![];
            let mut calculated_line = vec![];
            for raw_string in raw_line {
                raw_string_line.push(raw_string.clone());
                refs_line.push(vec![]);
                calculated_line.push(Value::Error);

                let (tree, refs) = Self::build_tree(raw_string);
                tree_line.push(tree);
                refs_to_line.push(refs);
            }
            raw_string_table.push(raw_string_line);
            tree_table.push(tree_line);
            refs_table.push(refs_line);
            refs_to_table.push(refs_to_line);
            calculated_table.push(calculated_line);
        }

        for y in 0..refs_table.len() {
            for x in 0..refs_table[y].len() {
                for (x_of_src, y_of_src) in &refs_to_table[y][x] {
                    let x_of_src = *x_of_src as usize;
                    let y_of_src = *y_of_src as usize;
                    if x != x_of_src || y != y_of_src {
                        refs_table[y_of_src][x_of_src].push((x, y));
                    }
                }
            }
        }

        for y in 0..tree_table.len() {
            for x in 0..tree_table[y].len() {
                Self::calc(x, y, &tree_table, &refs_table, &mut calculated_table);
            }
        }

        Ok(Table {
            raw_table: raw_string_table,
            tree_table,
            refs_table,
            calculated_table,
            current_pos: 0,
        })
    }
}
