use std::env::args;

use drafting::*;

fn main() {
    let mut args = args();
    let subcommand = args.nth(1);
    let app_config = read_app_config();

    match subcommand {
        Some(command) => {
            run_subcommand(&command, &app_config);
        }
        None => {
            run(&app_config);
        }
    }
}
