pub mod action;
pub mod cli;
pub mod io;
pub mod logging;

use std::{
    io::{stdin, BufRead, IsTerminal},
    path::PathBuf,
};

use anyhow::Result;
use home::home_dir;

use crate::action::action::*;
use crate::cli::cli::{cli, pipe_command};
use crate::io::env::{Environment, WContext};

use crate::logging::logging::{show_config, text};

fn main() -> Result<()> {
    let piped_commands = pipe_command()?;

    let matches = cli().get_matches();

    let wcontext: WContext = WContext {
        config_dir: format!(
            "{}/.config/wiki-o",
            home_dir().get_or_insert(PathBuf::new()).display()
        ),
    };
    let config = wcontext.config()?;
    let notes_dir = wcontext.notes_abs_dir()?;
    let metadata_dir = wcontext.metadata_abs_dir()?;
    let file_format: &String = &config.file_format;

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let content = sub_matches.get_one::<String>("NOTE").expect("required");
            let file_name = match sub_matches.get_one::<String>("FILE") {
                Some(file_name) => file_name.clone(),
                _ => "my_notes".to_string(),
            };

            add(content, &file_name, &notes_dir, file_format, &metadata_dir)?;
            Ok(())
        }
        Some(("show", sub_matches)) => {
            let file_name = sub_matches.get_one::<String>("FILE").expect("required");
            show(file_name, &notes_dir)?;
            Ok(())
        }
        Some(("list", sub_matches)) => {
            let is_short: bool = sub_matches.get_one::<String>("SHORT").is_some();

            list(is_short, &notes_dir)?;
            Ok(())
        }
        Some(("search", sub_matches)) => {
            let search_string = sub_matches
                .get_one::<String>("SEARCH_STRING")
                .expect("required");

            search(search_string, &metadata_dir)?;
            Ok(())
        }
        Some(("delete", sub_matches)) => {
            let file_name = sub_matches.get_one::<String>("FILE").expect("required");
            delete(&notes_dir, &metadata_dir, file_name, file_format)?;
            Ok(())
        }
        Some(("purge", _)) => {
            purge(&notes_dir, &metadata_dir)?;
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

                add(&content, &file_name, &notes_dir, file_format, &metadata_dir)?;
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

