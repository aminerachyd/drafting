pub mod commit;
pub mod config;
pub mod push;

use self::{
    commit::run_subcommand_commit, config::run_subcommand_config, push::run_subcommand_push,
};
use crate::AppConfig;
use std::process;

enum SubCommand {
    Config,
    Commit,
    Push,
    Unknown,
}

impl SubCommand {
    fn parse(subcommand: &str) -> Self {
        match subcommand {
            "config" => SubCommand::Config,
            "commit" => SubCommand::Commit,
            "push" => SubCommand::Push,
            _ => SubCommand::Unknown,
        }
    }
}

pub fn run_subcommand(subcommand: &str, app_config: &AppConfig) {
    let subcommand = SubCommand::parse(subcommand);

    match subcommand {
        SubCommand::Config => {
            run_subcommand_config(&app_config);
        }
        SubCommand::Commit => {
            run_subcommand_commit(&app_config);
        }
        SubCommand::Push => {
            run_subcommand_push(&app_config);
        }
        SubCommand::Unknown => {
            exit_unknown_command();
        }
    }
}

fn exit_unknown_command() {
    println!("Unknown command, exiting...");
    process::exit(1)
}
