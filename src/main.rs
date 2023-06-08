use std::{
    fs::{self},
    io::Error,
    os::unix::process::CommandExt,
    process,
};

use chrono::Datelike;

fn main() {
    let path = "/home/arachyd/drafts";

    let file_path = format!("{}/{}.md", path, latest_timestamp());

    if check_latest_file_exists(&file_path) == true {
        open_file_with_editor(&file_path).unwrap();
    } else {
        create_and_open_file_with_editor(&file_path).unwrap();
    }
}

// Check latest file, depends on day
fn check_latest_file_exists(path: &str) -> bool {
    match fs::File::open(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

// Get latest timestamp
fn latest_timestamp() -> String {
    let current_date = chrono::Utc::now();

    let day = current_date.day();
    let month = current_date.month();
    let year = current_date.year();

    format!("{}_{}_{}_draft", year, month, day)
}

fn open_file_with_editor(file_path: &str) -> Result<(), Error> {
    process::Command::new("vim").arg(&file_path).exec();

    Ok(())
}

fn create_and_open_file_with_editor(file_path: &str) -> Result<(), Error> {
    fs::File::create(file_path)?;

    open_file_with_editor(file_path)
}
