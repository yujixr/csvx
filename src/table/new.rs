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
        let mut raw_csv_table: Vec<Vec<String>> = vec![];

        for record in reader.records() {
            match record {
                Ok(record) => {
                    raw_csv_table.push(record.iter().map(|item| item.to_owned()).collect())
                }
                _ => return Err(Box::new(TableError::CSVParseError)),
            }
        }

        let mut raw_table = vec![];
        let mut tree_table = vec![];
        let mut ref_src_table = vec![];
        let mut dependents_table = vec![];
        let mut calculated_table = vec![];
        for raw_csv_line in raw_csv_table {
            let mut raw_line = vec![];
            let mut tree_line = vec![];
            let mut ref_src_line = vec![];
            let mut dependents_line = vec![];
            let mut calculated_line = vec![];
            for raw_item in raw_csv_line {
                let (tree, refs) = Self::build_tree(&raw_item);

                let mut dependents = HashMap::new();
                for dependent in refs {
                    *(dependents.entry(dependent).or_insert(0)) += 1;
                }

                raw_line.push(raw_item);
                ref_src_line.push(vec![]);
                dependents_line.push(dependents);
                tree_line.push(tree);
                calculated_line.push(Value::Error);
            }
            raw_table.push(raw_line);
            tree_table.push(tree_line);
            ref_src_table.push(ref_src_line);
            dependents_table.push(dependents_line);
            calculated_table.push(calculated_line);
        }

        for y in 0..ref_src_table.len() {
            for x in 0..ref_src_table[y].len() {
                for &(x_of_src, y_of_src) in dependents_table[y][x].keys() {
                    if y_of_src < ref_src_table.len() && x_of_src < ref_src_table[y].len() {
                        ref_src_table[y_of_src][x_of_src].push((x, y));
                    }
                }
            }
        }

        for y in 0..tree_table.len() {
            for x in 0..tree_table[y].len() {
                Self::calc(
                    x,
                    y,
                    &mut tree_table,
                    &mut ref_src_table,
                    &mut dependents_table,
                    &mut calculated_table,
                    &mut vec![],
                );
            }
        }

        Ok(Table {
            raw_table,
            tree_table,
            ref_src_table,
            dependents_table,
            calculated_table,
            current_pos_y: 0,
        })
    }
}
