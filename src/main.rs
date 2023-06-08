use drafting::*;

fn main() {
    let app_config = read_app_config();

    // TODO Read from an env variable/config ?
    let AppConfig {
        drafts_path,
        file_extension,
    } = app_config;

    // TODO Read extension to use for files ?
    let file_path = format!("{}/{}.{}", drafts_path, latest_timestamp(), file_extension);

    if check_file_exists(&file_path) == true {
        open_file_with_editor(&file_path).unwrap();
    } else {
        create_and_open_file_with_editor(&file_path).unwrap();
    }
}
