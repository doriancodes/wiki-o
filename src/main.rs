pub mod action;
pub mod cli;
pub mod env;
pub mod file;
pub mod logging;

use std::path::PathBuf;

use anyhow::Result;
use env::{Environment, WContext};
use home::home_dir;

use crate::logging::show_config;

fn main() -> Result<()> {
    let matches = cli::cli().get_matches();

    let wcontext: WContext = WContext {
        config_dir: format!("{}/.config/wiki-o", home_dir().get_or_insert(PathBuf::new()).display()),
    };
    let config = wcontext.config()?;
    let notes_dir = wcontext.notes_abs_dir()?;
    let file_format: &String = &config.file_format;

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let content = sub_matches.get_one::<String>("NOTE").expect("required");
            let file_name = match sub_matches.get_one::<String>("FILE") {
                Some(file_name) => file_name.clone(),
                _ => "my_notes".to_string(),
            };

            action::add(content, &file_name, &notes_dir, file_format)?;
            Ok(())
        }
        Some(("show", sub_matches)) => {
            let file_name = sub_matches.get_one::<String>("FILE").expect("required");
            action::show(file_name, &notes_dir)?;
            Ok(())
        }
        Some(("list", sub_matches)) => {
            let is_short: bool = match sub_matches.get_one::<String>("SHORT") {
                Some(_) => true,
                _ => false,
            };

            action::list(is_short, &notes_dir)?;
            Ok(())
        }
        Some(("delete", sub_matches)) => {
            let file_name = sub_matches.get_one::<String>("FILE").expect("required");
            action::delete(&notes_dir, &file_name, &file_format)?;
            Ok(())
        }
        Some(("purge", _)) => {
            action::purge(&notes_dir)?;
            Ok(())
        }
        Some(("config", _)) => {
            show_config("Current configuration: ".to_string(), config.to_string());
            Ok(())
        }
        _ => unreachable!(),
    }
}
