mod action;
mod cli;
mod config;
mod context;
mod costants;
mod file;

use anyhow::Context;

fn main() {
    let matches = cli::cli().get_matches();
    let context = context::Context::without_buffer();
    let notes_dir: &String = &context.initial_config.notes_abs_dir;
    let file_format: &String = &context.initial_config.file_format;

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let content = sub_matches.get_one::<String>("NOTE").expect("required");
            let file_name = match sub_matches.get_one::<String>("FILE") {
                Some(file_name) => file_name.clone(),
                _ => "my_notes".to_string(),
            };

            action::add(content, &file_name, notes_dir, file_format).unwrap(); //TODO handle nicely
        }
        Some(("list", sub_matches)) => {
            let is_short: bool = match sub_matches.get_one::<String>("SHORT") {
                Some(_) => true,
                _ => false,
            };

            action::list(is_short, notes_dir).unwrap(); //TODO handle nicely
        }
        Some(("delete", sub_matches)) => {
            let file_name = sub_matches
                .get_one::<String>("FILE")
                .context("file name is required")
                .unwrap();
            action::delete(notes_dir, &file_name, &file_format).unwrap(); //TODO handle nicely
        }
        Some(("purge", _)) => {
            action::purge(notes_dir, context.config_path).unwrap();
            //TODO handle nicely
        }
        Some(("config", _)) => {
            println!("Current configuration: \n\n{:#?}", context.initial_config);
            //TODO no debug
        }
        _ => unreachable!(),
    }
}
