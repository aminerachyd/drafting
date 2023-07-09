mod program;
mod subcommand;
mod util;

use program::run_program;
use subcommand::run_subcommand;
use util::app_config::{read_or_create_default_config_file, AppConfig};

pub fn run(subcommand: Option<String>) {
    let app_config = read_or_create_default_config_file();

    match subcommand {
        None => {
            run_program(&app_config);
        }
        Some(subcommand) => run_subcommand(&subcommand, &app_config),
    }
}
