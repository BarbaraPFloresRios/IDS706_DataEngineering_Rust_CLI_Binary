mod create;  // Import the create module
mod read;    // Import the read module
mod delete;  // Import the delete module
mod update;  // Import the update module

fn main() {
    let db_file = "data/WorldSmall.db";

    // CREATE: Call the function to create the database from a CSV file
    println!("\nPerforming CRUD operations on our database:");

    println!("\nCREATE: Creating the database from a CSV file...");
    if let Err(err) = create::create_database_from_csv("data/WorldSmall.csv", db_file) {
        eprintln!("Error creating the database: {}", err);
    } else {
        println!("Database created successfully.");
    }

    // UPDATE: Call the update function to transform the table (e.g., converting 'country' to uppercase)
    println!("\nUPDATE: Transforming the table (e.g., converting 'country' to uppercase)...");
    if let Err(err) = update::update_table(db_file) {
        eprintln!("Error updating the table: {}", err);
    } else {
        println!("Table updated successfully.");
    }

    // READ: Call the function to perform the read query
    println!("\nREAD: Querying the database...");
    if let Err(err) = read::query(db_file) {
        eprintln!("Error: {:?}", err);
    }

    // DELETE: Delete the database if needed
    println!("\nDELETE: Deleting the database...");
    if let Err(err) = delete::delete_database(db_file) {
        eprintln!("Error deleting the database: {}", err);
    } else {
        println!("\nDatabase deleted successfully.");
    }
}
