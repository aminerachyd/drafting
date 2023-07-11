use crate::{
    util::{change_dir, git_add_and_commit, today_timestamp},
    AppConfig,
};

pub fn run_subcommand_commit(app_config: &AppConfig) {
    let commit_message = format!("Drafts of {}", today_timestamp());

    chdir_and_commit_drafts(&app_config.config_file.drafts_path, commit_message);
}

fn chdir_and_commit_drafts(dir: &str, commit_message: String) {
    println!("Commiting draft files...");

    change_dir(dir);

    git_add_and_commit(&commit_message);
}
