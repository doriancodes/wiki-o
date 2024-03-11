use clap::{arg, Arg, Command};

pub fn cli() -> Command {
    Command::new("wiki-o")
        .about("Create a smart wiki from command line")
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("add")
                .about("Add note")
                .arg(arg!(<NOTE> "The note to write"))
                .arg_required_else_help(true)
                .arg(
                    Arg::new("FILE")
                        .short('f')
                        .long("file")
                        .value_name("FILE")
                        .help("The file name"),
                ),
        )
        .subcommand(
            Command::new("list").about("List all notes").arg(
                Arg::new("SHORT")
                    .short('s')
                    .long("short")
                    .help("List all notes file names")
                    .require_equals(false),
            ),
        )
        .subcommand(
            Command::new("delete").about("Delete a note").arg(
                Arg::new("FILE")
                    .short('f')
                    .long("file")
                    .value_name("FILE")
                    .help("The file name")
                    .require_equals(true),
            ),
        )
        .subcommand(
            Command::new("purge")
                .about("Purge all notes and wiki-o configuration for a clean slate"),
        )
        .subcommand(Command::new("config").about("Show wiki-o configuration"))
}
