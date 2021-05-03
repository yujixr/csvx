extern crate csvx;

use csvx::Table;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_csv = "pi,3^5,\"ref(0,0)\",-(1/0)
12%5,\"pow(3,5)\",0/NaN,\"\"\"Apollo\"\"\"
A1+A2,\"if(true , sqrt(25),round(if(false,1.1,2.5)))\",D2+1969,";
    println!("RAW CSV DATA:\n{}\n", raw_csv);

    let mut table = Table::new(raw_csv)?;
    println!("CALCULATED TABLE:\n{}", table);

    table.update(0, 0, "true")?;
    println!("UPDATED TABLE:\n{}", table.export_calculated_table()?);

    table.expand_y();
    println!("RAW TABLE:\n{}", table.export_raw_table()?);

    Ok(())
}
