// create.rs
use csv::Reader;
use rusqlite::Connection;
use std::error::Error;

pub fn create_database_from_csv(csv_file: &str, db_file: &str) -> Result<(), Box<dyn Error>> {
    // Connect to the SQLite database
    let conn = Connection::open(db_file)?;

    // Create a table in the database
    conn.execute(
        "CREATE TABLE IF NOT EXISTS my_table (country TEXT, region TEXT, gdppcap08 TEXT, polityiv TEXT)",
        [],
    )?;

    // Open the CSV file with error conversion
    let file = std::fs::File::open(csv_file)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    // Create a CSV reader
    let mut rdr = Reader::from_reader(file);

    // Iterate over the CSV rows and insert data into the database
    for result in rdr.records() {
        let record = result
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;  // Convert csv::Error to Box<dyn Error>

        let country: &str = &record[0];
        let region: &str = &record[1];
        let gdppcap08: &str = &record[2];  // Convert to &str
        let polityiv: &str = &record[3];   // Convert to &str

        // Insert data into the table
        conn.execute(
            "INSERT INTO my_table (country, region, gdppcap08, polityiv) VALUES (?, ?, ?, ?)",
            &[&country as &str, &region as &str, &gdppcap08 as &str, &polityiv as &str],
        )?;
    }

    Ok(())
}
