use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

use crate::costants;
use crate::file;

fn get_config(prod: bool) -> Result<Config> {
    let (config_path, config_file_name) = costants::get_env_config(prod);

    let config_file = file::format_file_name(&config_path, &config_file_name);

    let _config = file::read_from_file(&config_file)?;
    let config: Config = toml::from_str(&_config)?;
    Ok(config)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InitialConfig {
    pub notes_abs_dir: String,
    pub file_format: String,
}

impl InitialConfig {
    pub fn init() -> Result<InitialConfig> {
        let config = get_config(true)?;

        file::create_dir_if_not_exist(&config.notes_dir)?;
        Ok(InitialConfig {
            notes_abs_dir: config.notes_dir,
            file_format: config.file_format,
        })
    }
}

#[cfg(test)]
impl Default for InitialConfig {
    fn default() -> Self {
        let config = get_config(false).unwrap(); //TODO handle nicely

        InitialConfig {
            notes_abs_dir: config.notes_dir,
            file_format: config.file_format,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub notes_dir: String,
    pub file_format: String,
}
