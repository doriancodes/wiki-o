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
            Command::new("show")
                .about("Show files with similar name")
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
            Command::new("search")
                .about("Search notes for similar content")
                .arg(arg!(<SEARCH_STRING> "The search_string"))
                .arg_required_else_help(true),
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

#[cfg(test)]
mod tests {

    use super::cli;

    #[test]
    fn test_config_command() {
        let cli = cli();

        let m: clap::ArgMatches = cli.try_get_matches_from(["wiki-o", "add", "new"]).unwrap();

        let (_, add_cmd) = m.subcommand().unwrap();

        assert_eq!(add_cmd.get_one::<String>("NOTE").unwrap(), "new");
    }
}
