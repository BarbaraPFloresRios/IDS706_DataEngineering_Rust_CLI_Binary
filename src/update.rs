use rusqlite::{Connection, Result};
use rusqlite::params;

pub fn update_table(db_file: &str) -> Result<()> {
    let conn = Connection::open(db_file)?;

    // Perform the transformation in the table
    conn.execute(
        "UPDATE my_table SET country = UPPER(country)",
        params![] // There are no parameters for this query
    )?;

    conn.execute(
        "UPDATE my_table SET region = UPPER(region)",
        params![] // There are no parameters for this query
    )?;

    Ok(())
}
