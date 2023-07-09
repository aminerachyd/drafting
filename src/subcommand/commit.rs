use std::{env, process};

use crate::{util::today_timestamp, AppConfig};

pub fn run_subcommand_commit(app_config: &AppConfig) {
    let commit_message = format!("Drafts of {}", today_timestamp());

    chdir_and_commit_drafts(&app_config, commit_message);
}

fn chdir_and_commit_drafts(app_config: &AppConfig, commit_message: String) {
    println!("Commiting draft files...");

    let drafts_dir = &app_config.config_file.drafts_path;

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
