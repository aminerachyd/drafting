use std::{
    fs::{self, File},
    io::{ErrorKind, Read, Write},
    process,
};

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

pub fn read_or_create_default_config_file() -> AppConfig {
    let home = env!("HOME");
    let config_file_path = format!("{}/.config/drafting.yaml", home);

    let config_file = fs::File::open(config_file_path.clone());

    match config_file {
        Ok(f) => parse_config(f, config_file_path),

        Err(e) => match e.kind() {
            ErrorKind::NotFound => create_default_config(),
            _ => {
                println!("Error reading config file, exiting...");
                process::exit(1)
            }
        },
    }
}

fn parse_config(file: File, config_file_path: String) -> AppConfig {
    let editor = std::env::var("EDITOR");

    let config_string = read_file_content(file);

    let result = serde_yaml::from_str::<ConfigFile>(&config_string);
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

fn read_file_content(mut file: File) -> String {
    let mut buffer = "".to_owned();

    file.read_to_string(&mut buffer).unwrap();

    buffer
}

fn create_default_config() -> AppConfig {
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
