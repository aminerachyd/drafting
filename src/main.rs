use std::env::args;

use drafting::*;

fn main() {
    let mut args = args();
    let subcommand = args.nth(1);
    match subcommand {
        Some(command) => {
            run_subcommand(&command);
        }
        None => {
            let app_config = read_app_config();

            let AppConfig {
                editor,
                drafts_path,
                file_extension,
            } = app_config;

            check_draft_path(&drafts_path);

            let file_path = format!("{}/{}.{}", drafts_path, latest_timestamp(), file_extension);

            if check_file_exists(&file_path) == true {
                open_file_with_editor(&file_path, &editor).unwrap();
            } else {
                create_and_open_file_with_editor(&file_path, &editor).unwrap();
            }
        }
    }
}
