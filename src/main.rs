pub mod cli;
pub mod core;
pub mod io;
pub mod logging;

use anyhow::Result;

use io::env::{ContextWriter, WEnv};

use crate::cli::cmd::{cli, pipe_command};
use crate::core::action::*;

use crate::logging::logger::{show_config, text};

fn main() -> Result<()> {
    let piped_commands = pipe_command()?;

    let matches = cli().get_matches();

    let current_env = WEnv::Prod;

    let init_dir = ContextWriter { env: WEnv::Prod };

    init_dir.init()?;

    let config = current_env.config();
    let file_format: &String = &config.file_format;

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let content = sub_matches.get_one::<String>("NOTE").expect("required");
            let file_name = match sub_matches.get_one::<String>("FILE") {
                Some(file_name) => file_name.clone(),
                _ => "my_notes".to_string(),
            };

            // let note = WNote::from_env(content.clone(), file_name.clone(), &WEnv::Prod);

            let _wfile = add(content, &file_name, file_format, &current_env)?;

            Ok(())
        }
        Some(("show", sub_matches)) => {
            let file_name = sub_matches.get_one::<String>("FILE").expect("required");
            show(file_name, &current_env)?;
            Ok(())
        }
        Some(("list", sub_matches)) => {
            let is_short: bool = sub_matches.get_one::<String>("SHORT").is_some();

            list(is_short, &current_env)?;
            Ok(())
        }
        Some(("search", sub_matches)) => {
            let search_string = sub_matches
                .get_one::<String>("SEARCH_STRING")
                .expect("required");

            search(search_string, &current_env)?;
            Ok(())
        }
        Some(("delete", sub_matches)) => {
            let file_name = sub_matches.get_one::<String>("FILE").expect("required");
            delete(file_name, file_format, &current_env)?;
            Ok(())
        }
        Some(("purge", _)) => {
            purge(&current_env)?;
            Ok(())
        }
        Some(("pa", sub_matches)) => {
            if piped_commands.is_empty() {
                text("No piped command provided");
            } else {
                let content = piped_commands;
                let file_name = match sub_matches.get_one::<String>("FILE") {
                    Some(file_name) => file_name.clone(),
                    _ => "my_notes".to_string(),
                };

                add(&content, &file_name, file_format, &current_env)?;
            }
            Ok(())
        }
        Some(("config", _)) => {
            show_config("Current configuration: ".to_string(), config.to_string());
            Ok(())
        }
        _ => unreachable!(),
    }
}
