use std::{
    env,
    fs::{self, File},
    io::{Error, ErrorKind, Read, Write},
    os::unix::process::CommandExt,
    process,
};

use chrono::Datelike;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    pub editor: String,
    pub drafts_path: String,
    pub file_extension: String,
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub config_file_path: String,
    pub config_file: ConfigFile,
}

// Runs the program
pub fn run(app_config: &AppConfig) {
    check_draft_path(&app_config);

    let latest_file_path = latest_file_path(&app_config);

    if check_file_exists(&latest_file_path) == true {
        open_file_with_editor(&latest_file_path, &app_config).unwrap();
    } else {
        create_and_open_file_with_editor(&latest_file_path, &app_config).unwrap();
    }
}

// Runs subcommand
pub fn run_subcommand(command: &str, app_config: &AppConfig) {
    if command.eq("config") {
        open_config_file(&app_config);
    } else if command.eq("commit") {
        commit_drafts(&app_config);
    } else {
        println!("Unknown command, exiting...");
        process::exit(1)
    }
}

/**
 * Subcommand: Opens config file for editing
 */
fn open_config_file(app_config: &AppConfig) {
    println!("Opening config file...");

    if open_file_with_editor(&app_config.config_file_path, app_config).is_err() {
        println!("Config file doesn't exist, creating one...");
        create_and_open_file_with_editor(&app_config.config_file_path, app_config).unwrap();
    }
}

/**
 * Subcommand: Commits drafts
 */
fn commit_drafts(app_config: &AppConfig) {
    println!("Commiting draft files...");

    let drafts_dir = &app_config.config_file.drafts_path;
    let commit_message = format!("Drafts of {}", latest_timestamp());

    env::set_current_dir(drafts_dir).unwrap();

    process::Command::new("git")
        .args(["add", "."])
        .spawn()
        .unwrap();

    process::Command::new("git")
        .args(["commit", "-m", &commit_message])
        .spawn()
        .unwrap();
}

impl AppConfig {
    fn create_default_config() -> Self {
        let home = env!("HOME");
        let config_file_path = format!("{}/.config/drafting.yaml", home);

        println!("Writing default config to {}", config_file_path);

        let config_file = ConfigFile {
            editor: "vim".to_owned(),
            drafts_path: format!("{}/drafts", home),
            file_extension: "md".to_owned(),
        };

        let yaml = serde_yaml::to_string(&config_file).unwrap();

        let app_config = AppConfig {
            config_file_path,
            config_file,
        };

        let mut f = fs::File::create(&app_config.config_file_path).unwrap();

        let result = f.write_all(yaml.as_bytes());

        match result {
            Ok(_) => {}
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    println!("~/.config directory not existing, exiting...");
                    process::exit(1)
                }
                _ => {
                    println!("Error writing config file, exiting...");
                    process::exit(1)
                }
            },
        }

        app_config
    }

    fn parse_config(mut f: File, config_file_path: String) -> Self {
        let mut buffer = "".to_owned();

        let editor = std::env::var("EDITOR");

        f.read_to_string(&mut buffer).unwrap();

        let result = serde_yaml::from_str::<ConfigFile>(&buffer);
        match result {
            Err(_) => {
                println!("Failed to parse config file, probably wrong format, exiting...");
                process::exit(1)
            }
            Ok(config_file) => match editor {
                Ok(editor) => AppConfig {
                    config_file_path,
                    config_file: ConfigFile {
                        editor,
                        ..config_file
                    },
                },
                Err(_) => AppConfig {
                    config_file_path,
                    config_file,
                },
            },
        }
    }
}

// Reads config file, used by application during startup
pub fn read_app_config() -> AppConfig {
    let home = env!("HOME");
    let config_file_path = format!("{}/.config/drafting.yaml", home);

    let config_file = fs::File::open(config_file_path.clone());

    match config_file {
        Ok(f) => AppConfig::parse_config(f, config_file_path),

        Err(e) => match e.kind() {
            ErrorKind::NotFound => AppConfig::create_default_config(),
            _ => {
                println!("Error reading config file, exiting...");
                process::exit(1)
            }
        },
    }
}

// Check file existance
fn check_file_exists(path: &str) -> bool {
    match fs::File::open(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

// Check drafts directory and create it if it doesn't exist
fn check_draft_path(app_config: &AppConfig) {
    let drafts_path = &app_config.config_file.drafts_path;
    let result = fs::read_dir(drafts_path);
    match result {
        Ok(_) => {}
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("Drafts directory doesn't exist, creating it...");
                fs::create_dir(drafts_path).unwrap();
            }
            _ => {
                println!("Error reading drafts directory, exiting...");
                process::exit(1)
            }
        },
    }
}

// Get path of latest file type
fn latest_file_path(app_config: &AppConfig) -> String {
    format!(
        "{}/{}.{}",
        app_config.config_file.drafts_path,
        latest_timestamp(),
        app_config.config_file.file_extension
    )
}

fn open_file_with_editor(file_path: &str, app_config: &AppConfig) -> Result<(), Error> {
    process::Command::new(&app_config.config_file.editor)
        .arg(&file_path)
        .exec();

    Ok(())
}

// Creates and opens file with editor in question
fn create_and_open_file_with_editor(file_path: &str, app_config: &AppConfig) -> Result<(), Error> {
    fs::File::create(file_path)?;

    open_file_with_editor(file_path, app_config)
}

// Get latest timestamp
fn latest_timestamp() -> String {
    let current_date = chrono::Utc::now();

    let day = current_date.day();
    let month = current_date.month();
    let year = current_date.year();

    format!("{}_{}_{}_draft", year, month, day)
}
