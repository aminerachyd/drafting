pub mod commit;
pub mod config;

use self::{commit::run_subcommand_commit, config::run_subcommand_config};
use crate::AppConfig;
use std::process;

pub fn run_subcommand(command: &str, app_config: &AppConfig) {
    match command {
        "config" => {
            run_subcommand_config(&app_config);
        }
        "commit" => {
            run_subcommand_commit(&app_config);
        }
        _ => {
            exit_unknown_command();
        }
    }
}

fn exit_unknown_command() {
    println!("Unknown command, exiting...");
    process::exit(1)
}
