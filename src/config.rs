use std::fs;

use serde_derive::{Deserialize, Serialize};

fn create_dirs(notes_dir: &String, config_dir: &String) {
    if fs::metadata(&notes_dir).is_err() {
        fs::create_dir_all(&notes_dir).unwrap();
    }
    if fs::metadata(&config_dir).is_err() {
        fs::create_dir_all(&config_dir).unwrap();
    }
}

fn set_config() {
    let config_path = format!(
        "{}/.config/wiki-o",
        home::home_dir().unwrap().display()
    );
    if fs::metadata(&config_path).is_err() {
        fs::create_dir_all(&config_path).unwrap();
        let config_file = format!("{}/config.toml", config_path);
        let config = Config {
            directories: Directories {
                notes: String::from("wiki-o/.notes"),
                config: String::from("wiki-o/.config"),
            },
            editor: Editor {
                format: String::from("md"),
            },
        };
        let _config = toml::to_string(&config).unwrap();
        fs::write(config_file, _config).unwrap();
    }
}

fn get_config() -> Config {
    let _config = fs::read_to_string(
      format!(
         "{}/.config/wiki-o/config.toml",
         home::home_dir().unwrap().display()
     )).expect("Unable to read file");
    return toml::from_str(&_config).unwrap();
}

pub struct InitialConfig {
    pub notes_dir: String,
    pub notes_abs_dir: String,
    pub config_dir: String,
    pub config_abs_dir: String,
    pub file_format: String,
}

impl InitialConfig {
    pub fn init() -> InitialConfig {
        set_config();
        let config = get_config();
        let notes_dir = config.directories.notes;
        let notes_abs_dir = format!("{}/{}", home::home_dir().unwrap().display(), notes_dir);
        let config_dir = config.directories.config;
        let config_abs_dir = format!("{}/{}", home::home_dir().unwrap().display(), config_dir);
        let file_format = config.editor.format;

        create_dirs(&notes_abs_dir, &config_abs_dir);
        return InitialConfig {
            notes_dir,
            notes_abs_dir,
            config_dir,
            config_abs_dir,
            file_format,
        };
    }
}

#[derive(Deserialize, Serialize)]
struct Editor {
    format: String,
}

#[derive(Deserialize, Serialize)]
struct Directories {
    notes: String,
    config: String,
}

#[derive(Deserialize, Serialize)]
struct Config {
    directories: Directories,
    editor: Editor,
}
