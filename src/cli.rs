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
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .value_name("FILE")
                        .help("The file name")
                        .default_missing_value("file.txt")
                        .require_equals(false),
                ),
        )
        .subcommand(Command::new("list").about("List all notes"))
        .subcommand(
            Command::new("delete").about("Purge all notes").arg(
                Arg::new("all")
                    .short('a')
                    .long("all")
                    .help("Purge all notes and delete all wiki-o files")
                    .default_missing_value("false")
                    .require_equals(false),
            ),
        )
        .subcommand(Command::new("init").about("Initialize wiki-o configuration"))
}
