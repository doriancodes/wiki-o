mod action;
mod cli;
mod config;
mod csv;

fn main() {
    let matches = cli::cli().get_matches();
    let config = config::InitialConfig::init();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            action::add(sub_matches, config);
        }
        Some(("list", sub_matches)) => {
            action::list(config);
        }
        Some(("delete", _)) => action::delete(config),
        Some(("init", _)) => {
            action::init();
            println!("Config initialized");
        }
        _ => unreachable!(),
    }
}
