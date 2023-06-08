use std::{
    env,
    fs::{self, File},
    io::{Error, ErrorKind, Read, Write},
    os::unix::process::CommandExt,
    process::{self, exit},
};

use chrono::Datelike;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub drafts_path: String,
    pub file_extension: String,
}

impl AppConfig {
    fn default_config() -> Self {
        let HOME = env!("HOME");
        let config_file_path = format!("{}/.config/drafting.yaml", HOME);

        let app_config = AppConfig {
            drafts_path: format!("{}/drafts", HOME),
            file_extension: "md".to_owned(),
        };

        let yaml = serde_yaml::to_string(&app_config).unwrap();

        println!("Writing default config to {}", config_file_path);

        let mut f = fs::File::create(config_file_path).unwrap();

        let result = f.write_all(yaml.as_bytes());

        match result {
            Ok(_) => {}
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    println!("~/.config directory not existing, exiting...");
                    process::exit(1)
                    // fs::create_dir("~/.config").unwrap();
                }
                _ => {
                    println!("Error writing config file, exiting...");
                    process::exit(1)
                }
            },
        }

        app_config
    }

    fn parse_config(mut f: File) -> Self {
        let mut buffer = "".to_owned();

        f.read_to_string(&mut buffer).unwrap();

        serde_yaml::from_str(&buffer).unwrap()
    }
}

// Reads config file, used by application during startup
pub fn read_app_config() -> AppConfig {
    let HOME = env!("HOME");
    let config_file_path = format!("{}/.config/drafting.yaml", HOME);

    let config_file = fs::File::open(config_file_path);

    match config_file {
        Ok(f) => AppConfig::parse_config(f),

        Err(e) => match e.kind() {
            ErrorKind::NotFound => AppConfig::default_config(),
            _ => {
                println!("Error reading config file, exiting...");
                process::exit(1)
            }
        },
    }
}

// Check file existance
pub fn check_file_exists(path: &str) -> bool {
    match fs::File::open(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

// Check drafts directory and create it if it doesn't exist
pub fn check_draft_path(path: &str) {
    let result = fs::read_dir(path);
    match result {
        Ok(_) => {}
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("Drafts directory doesn't exist, creating it...");
                fs::create_dir(path).unwrap();
            }
            _ => {
                println!("Error reading drafts directory, exiting...");
                process::exit(1)
            }
        },
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
