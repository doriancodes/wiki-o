use std::fs;
use std::{fs::File, io::BufWriter};

use crate::costants;
use crate::{config, file};

fn env_config_file(prod: bool) -> (String, String) {
    let (config_path, config_file_name) = costants::get_env_config(prod);
    let config_file = file::format_file_name(&config_path, &config_file_name);

    if fs::metadata(&config_path).is_err() {
        fs::create_dir_all(&config_path).unwrap();
        let notes_dir = if prod {
            String::from("wiki-o/notes")
        } else {
            String::from("test-dir/notes")
        };
        let config = config::Config {
            notes_dir: notes_dir,
            file_format: String::from("md"),
        };
        let _config = toml::to_string(&config).unwrap();
        fs::write(&config_file, _config).unwrap();
        println!("Created config file: {}", &config_file);
    }

    (config_path, config_file_name)
}

pub struct Context {
    pub initial_config: config::InitialConfig,
    pub file_buffer: Option<BufWriter<File>>,
    pub config_file_name: String,
    pub config_path: String,
}

impl Context {
    pub fn with_buffer(file_path: String) -> Context {
        let file = File::create(file_path).unwrap(); //TODO handle nicely
        let file_buffer = BufWriter::new(file);
        let (config_path, config_file_name) = env_config_file(true);

        Context {
            initial_config: config::InitialConfig::init().unwrap(),
            file_buffer: Some(file_buffer),
            config_file_name,
            config_path,
        }
    }

    pub fn without_buffer() -> Context {
        let (config_path, config_file_name) = env_config_file(true);

        Context {
            initial_config: config::InitialConfig::init().unwrap(),
            file_buffer: None,
            config_file_name,
            config_path,
        }
    }
}

#[cfg(test)]
impl Default for Context {
    fn default() -> Self {
        let (config_path, config_file_name) = env_config_file(false);
        let initial_config = config::InitialConfig::default();

        Context {
            initial_config,
            file_buffer: None,
            config_file_name,
            config_path,
        }
    }
}
