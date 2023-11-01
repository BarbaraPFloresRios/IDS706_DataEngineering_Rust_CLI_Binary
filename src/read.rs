extern crate rusqlite;
extern crate prettytable;

use rusqlite::Connection;
use prettytable::{Table, Row, Cell};

pub fn print_table(_cursor: &rusqlite::Statement, data: Vec<Vec<String>>) {
    let mut table = Table::new();
    for row in data.iter() {
        let cells: Vec<Cell> = row.iter().map(|value| Cell::new(value)).collect();
        table.add_row(Row::new(cells));
    }
    table.printstd();
}


pub fn query(db_file: &str) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(db_file)?;

    // Query 1
    let mut cursor = conn.prepare("SELECT * FROM my_table ORDER BY RANDOM() LIMIT 5")?;
    let rows = cursor
        .query_map([], |row| {
            let mut values = Vec::new();
            for i in 0..row.column_count() {
                values.push(row.get(i)?);
            }
            Ok(values)
        })?
        .collect::<Result<Vec<Vec<String>>, rusqlite::Error>>()?;

    println!("\nLet's quickly review our database. Let's take a sample of how it is constructed.");
    print_table(&cursor, rows);

    // Query 2
    let mut cursor = conn.prepare("SELECT region, COUNT(*) AS N FROM my_table GROUP BY region")?;
    let rows = cursor
        .query_map([], |row| {
            let region: String = row.get(0)?;
            let count: i64 = row.get(1)?;
            Ok((region, count))
        })?
        .collect::<Result<Vec<(String, i64)>, rusqlite::Error>>()?;

    println!("\nHow many records per continent does our database have?");
    print_table(&cursor, rows.iter().map(|(r, c)| vec![r.clone(), c.to_string()]).collect());


    Ok(())
}