mod action;
mod cli;
mod config;
mod csv;

fn main() {
    let matches = cli::cli().get_matches();
    let config = config::InitialConfig::init(); // TODO when deamon mutable

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let content = sub_matches.get_one::<String>("NOTE").expect("required");
            let file_name = match sub_matches.get_one::<String>("FILE") {
                Some(file_name) => file_name.clone(),
                _ => "my_notes".to_string(),
            };
            let notes_dir: &String = &config.notes_abs_dir;
            let config_dir = &config.config_abs_dir;
            let file_format: &String = &config.file_format;

            action::add(content, &file_name, notes_dir, config_dir, file_format);
        }
        Some(("list", sub_matches)) => {
            let is_short: bool = match sub_matches.get_one::<String>("SHORT") {
                Some(_) => true,
                _ => false,
            };
            let notes_dir: &String = &config.notes_abs_dir;

            action::list(is_short, notes_dir);
        }
        Some(("delete", _)) => {
            let notes_abs_dir = &config.notes_abs_dir;
            let config_abs_dir = &config.config_abs_dir;

            action::delete(notes_abs_dir, config_abs_dir)
        }
        Some(("config", _)) => {
            println!("Current configuration: \n\n{:#?}", config);
        }
        _ => unreachable!(),
    }
}
