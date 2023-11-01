use std::thread;
use std::time::Duration;

// delete.rs
use std::fs;

pub fn delete_database(db_file: &str) -> Result<(), std::io::Error> {
    let duracion = Duration::from_secs(4);
    thread::sleep(duracion);
    if fs::metadata(db_file).is_ok() {
        fs::remove_file(db_file)?;
    }
    Ok(())
}


