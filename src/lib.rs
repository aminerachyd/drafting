use std::{
    fs::{self, File},
    io::{Error, ErrorKind},
    os::unix::process::CommandExt,
    process,
};

use chrono::Datelike;

pub struct AppConfig {
    pub drafts_path: String,
    pub file_extension: String,
}

impl AppConfig {
    fn default_config() -> Self {
        AppConfig {
            drafts_path: "~/drafts".to_owned(),
            file_extension: ".md".to_owned(),
        }
    }

    fn parse_config(f: File) -> Self {
        unimplemented!()
    }
}

// Reads config file, used by application during startup
pub fn read_app_config() -> AppConfig {
    let config_file = fs::File::open("~/.config/draftings.yaml");

    match config_file {
        Ok(f) => AppConfig::parse_config(f),

        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                AppConfig::default_config()
            } else {
                println!("Error reading config file, exiting...");
                process::exit(1)
            }
        }
    }
}

// Check file existance
pub fn check_file_exists(path: &str) -> bool {
    match fs::File::open(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

// Get latest timestamp
pub fn latest_timestamp() -> String {
    let current_date = chrono::Utc::now();

    let day = current_date.day();
    let month = current_date.month();
    let year = current_date.year();

    format!("{}_{}_{}_draft", year, month, day)
}

pub fn open_file_with_editor(file_path: &str) -> Result<(), Error> {
    // TODO Using vim as editor, make is customizable ?
    process::Command::new("vim").arg(&file_path).exec();

    Ok(())
}

pub fn create_and_open_file_with_editor(file_path: &str) -> Result<(), Error> {
    fs::File::create(file_path)?;

    open_file_with_editor(file_path)
}
