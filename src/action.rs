use std::fs;
use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use crate::config;
use crate::config::InitialConfig;
use crate::csv;

pub fn init() -> InitialConfig {
    return config::InitialConfig::init();
}

pub fn add(sub_matches: &clap::ArgMatches, config: config::InitialConfig) {
    let notes_dir: &String = &config.notes_abs_dir;

    let content = sub_matches.get_one::<String>("NOTE").expect("required");
    let content_f = format!("{}\n\n", content);

    let file_name = match sub_matches.get_one::<String>("FILE") {
        Some(file_name) => file_name.clone(),
        _ => "my_notes".to_string(),
    };

    let file_path = format!("{}/{}.{}", notes_dir, file_name, config.file_format);

    csv::write_to_csv(&file_name, file_path.clone(), config).unwrap();

    let mut note = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .create(true)
        .open(file_path)
        .unwrap();
    note.seek(SeekFrom::Start(0)).unwrap();
    note.write_all(content_f.as_bytes()).unwrap();

    println!("Added {} to {}", content, file_name);
}

pub fn list(sub_matches: &clap::ArgMatches, config: config::InitialConfig) {
    let notes_dir: String = config.notes_abs_dir;

    let paths = fs::read_dir(&notes_dir).unwrap();

    for path in paths {
        let path_i = path.unwrap().path().display().to_string();
        println!("File: {}", path_i);
        match sub_matches.get_one::<String>("SHORT") {
            Some(flag)  => {
                if flag == "short" {
                    continue;
                }
            }
            _ => {
                let content = fs::read_to_string(path_i).expect("unable to read file");
                print!("\n{}\n", content);
            },
        }

    }
}

pub fn delete(config: config::InitialConfig) {
    fs::remove_dir_all(config.notes_abs_dir).unwrap();
    fs::remove_dir_all(config.config_abs_dir).unwrap();
    println!("Deleted all notes");
}
