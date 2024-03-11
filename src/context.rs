use std::{fs::File, io::BufWriter};

use crate::config;

pub struct Context {
    pub initial_config: config::InitialConfig,
    pub file_buffer: Option<BufWriter<File>>,
    pub config_file: String,
}

impl Context {
    pub fn with_buffer(file_path: String, config_file: String) -> Context {
        let file = File::create(file_path).unwrap(); //TODO handle nicely
        let file_buffer = BufWriter::new(file);
        Context {
            initial_config: config::InitialConfig::init().unwrap(),
            file_buffer: Some(file_buffer),
            config_file,
        }
    }

    pub fn without_buffer(config_file: String) -> Context {
        Context {
            initial_config: config::InitialConfig::init().unwrap(),
            file_buffer: None,
            config_file,
        }
    }
}

#[cfg(test)]
impl Default for Context {
    fn default() -> Self {
        let initial_config = config::InitialConfig::default();
        let current_dir = std::env::current_dir().unwrap(); //TODO handle nicely

        let config_file = format!("{}/test-dir/config/config.toml", current_dir.display());
        Context {
            initial_config,
            file_buffer: None,
            config_file,
        }
    }
}
