use std::io::{stdin, BufRead, IsTerminal};

use anyhow::Result;
use clap::{arg, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wiki-o", author, version, about, arg_required_else_help = true)]
/// Create a smart wiki from command line
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add note
    Add {
        /// The note to write
        #[arg(value_name = "NOTE")]
        note: String,
        /// file name to add
        #[arg(short, long)]
        file: Option<String>,
    },
    /// Show files with similar name
    Show {
        /// file name to show
        #[arg(short, long)]
        file: String,

        /// True if showing with file name
        #[arg(short, long)]
        complete: Option<bool>,
    },
    /// List all notes
    List {
        /// list only file names
        #[arg(short, long)]
        short: Option<bool>,
    },
    /// Search notes for similar content
    Search {
        /// lists test values
        #[arg(value_name = "SEARCH_STRING")]
        search_string: String,
    },
    /// Delete a note
    Delete {
        /// file to delete
        #[arg(short, long)]
        file: String,
    },
    /// Purge all notes
    Purge {},
    /// Piped add note
    Pa {
        /// file name to add
        #[arg(short, long)]
        file: Option<String>,
    },
    /// Show wiki-o configuration
    Config {},
}

pub fn pipe_command() -> Result<String> {
    let mut input = String::new();
    loop {
        let mut buffer = String::new();
        if stdin().is_terminal() {
            break;
        }
        match stdin().lock().read_line(&mut buffer) {
            Ok(len) => {
                if len == 0 {
                    break;
                } else {
                    input.push_str(&buffer);
                }
            }
            Err(_) => {
                break;
            }
        }
    }
    Ok(input)
}

// #[cfg(test)]
// mod tests {

//     use super::cli;

//     #[test]
//     fn test_config_command() {
//         let cli = cli();

//         let m: clap::ArgMatches = cli.try_get_matches_from(["wiki-o", "add", "new"]).unwrap();

//         let (_, add_cmd) = m.subcommand().unwrap();

//         assert_eq!(add_cmd.get_one::<String>("NOTE").unwrap(), "new");
//     }
// }
