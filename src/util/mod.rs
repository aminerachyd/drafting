pub mod app_config;

use std::{fs, io::Error, os::unix::process::CommandExt, process};

use chrono::Datelike;

pub fn create_and_open_file_with_editor(file_path: &str, editor: &str) -> Result<(), Error> {
    fs::File::create(file_path)?;

    open_file_with_editor(file_path, editor)
}

pub fn open_file_with_editor(file_path: &str, editor: &str) -> Result<(), Error> {
    process::Command::new(editor).arg(&file_path).exec();

    Ok(())
}

pub fn today_timestamp() -> String {
    let current_date = chrono::Utc::now();

    let day = current_date.day();
    let month = current_date.month();
    let year = current_date.year();

    format!("{}_{}_{}", year, month, day)
}
