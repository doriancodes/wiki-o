pub mod cli;
pub mod core;
pub mod io;
pub mod logging;

use anyhow::{Ok, Result};

use clap::Parser as _;
use cli::cmd::Commands;
use io::env::{ContextWriter, WEnv};

use crate::cli::cmd::{pipe_command, Cli};
use crate::core::action::*;

use crate::logging::logger::show_config;

fn main() -> Result<()> {
    let piped_commands = pipe_command()?;

    let cli = Cli::parse();

    let current_env = WEnv::Prod;

    let init_dir = ContextWriter { env: WEnv::Prod };

    init_dir.init()?;

    let config = current_env.config();
    let file_format: &String = &config.file_format;

    match &cli.command {
        Some(Commands::Add { note, file }) => {
            let file_name = match file {
                Some(file_name) => file_name.clone(),
                _ => "my_notes".to_string(),
            };
            add(note, &file_name, file_format, &current_env)?;
            Ok(())
        }
        Some(Commands::Show { file, complete }) => {
            let is_complete = complete.is_some();
            show(file, &is_complete, &current_env)?;

            Ok(())
        }
        Some(Commands::List { short }) => {
            let is_short = short.unwrap_or(false);
            list(is_short, &current_env)?;
            Ok(())
        }
        Some(Commands::Search { search_string }) => {
            search(search_string, &current_env)?;
            Ok(())
        }
        Some(Commands::Delete { file }) => {
            delete(file, file_format, &current_env)?;
            Ok(())
        }
        Some(Commands::Purge {}) => {
            purge(&current_env)?;
            Ok(())
        }
        Some(Commands::Pa { file }) => {
            let file_name = match file {
                Some(file_name) => file_name.clone(),
                _ => "my_notes".to_string(),
            };
            add(&piped_commands, &file_name, file_format, &current_env)?;
            Ok(())
        }
        Some(Commands::Config {}) => {
            show_config("Current configuration: ".to_string(), config.to_string());
            Ok(())
        }
        _ => unreachable!(),
    }
}
