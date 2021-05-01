use super::*;

impl Table {
    /// Export the calculated table as CSV.
    pub fn export_calculated_table(&self) -> Result<String, Box<dyn Error>> {
        let mut writer = csv::Writer::from_writer(vec![]);

        for line in &self.calculated_table {
            let mut line_record = vec![];
            for item in line {
                line_record.push(format!("{}", item));
            }
            writer.write_record(line_record)?;
        }
        Ok(String::from_utf8(writer.into_inner()?)?)
    }

    /// Export as CSVX.
    pub fn export_raw_table(&self) -> Result<String, Box<dyn Error>> {
        let mut writer = csv::Writer::from_writer(vec![]);

        for line in &self.raw_table {
            let mut line_record = vec![];
            for item in line {
                line_record.push(item);
            }
            writer.write_record(line_record)?;
        }
        Ok(String::from_utf8(writer.into_inner()?)?)
    }
}
