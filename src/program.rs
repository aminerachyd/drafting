use std::{fs, io::ErrorKind, process};

use crate::{
    util::{create_and_open_file_with_editor, open_file_with_editor, today_timestamp},
    AppConfig,
};

pub fn run_program(app_config: &AppConfig) {
    check_or_create_drafts_path(&app_config);

    let editor = &app_config.config_file.editor;

    let today_draft_path = format_today_draft_path(&app_config);

    if check_if_file_exists(&today_draft_path) == true {
        open_file_with_editor(&today_draft_path, editor).unwrap();
    } else {
        create_and_open_file_with_editor(&today_draft_path, editor).unwrap();
    }
}

fn check_or_create_drafts_path(app_config: &AppConfig) {
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

fn format_today_draft_path(app_config: &AppConfig) -> String {
    format!(
        "{}/{}.{}",
        app_config.config_file.drafts_path,
        today_draft_name(),
        app_config.config_file.file_extension
    )
}

fn check_if_file_exists(path: &str) -> bool {
    match fs::File::open(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn today_draft_name() -> String {
    format!("{}_{}", today_timestamp(), "draft")
}
