use crate::util::{app_config::AppConfig, change_dir, git_push};

pub fn run_subcommand_push(app_config: &AppConfig) {
    println!("Pushing commits to remote repository if configured...");

    change_dir(&app_config.config_file.drafts_path);

    git_push();
}
