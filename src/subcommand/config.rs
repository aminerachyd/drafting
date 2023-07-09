use crate::{
    util::{create_and_open_file_with_editor, open_file_with_editor},
    AppConfig,
};

pub fn run_subcommand_config(app_config: &AppConfig) {
    edit_or_create_config_file(&app_config);
}

fn edit_or_create_config_file(app_config: &AppConfig) {
    println!("Opening config file...");

    let config_file_path = &app_config.config_file_path;
    let editor = &app_config.config_file.editor;

    if open_file_with_editor(config_file_path, editor).is_err() {
        println!("Config file doesn't exist, creating one...");
        create_and_open_file_with_editor(config_file_path, editor).unwrap();
    }
}
