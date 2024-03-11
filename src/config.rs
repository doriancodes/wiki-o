use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use std::fs;

fn create_notes_dir(notes_dir: &String) -> Result<()> {
    if fs::metadata(&notes_dir).is_err() {
        fs::create_dir_all(&notes_dir)?;
    }

    Ok(())
}

fn set_config() -> Result<()> {
    let config_path = format!("{}/.config/wiki-o", home::home_dir().unwrap().display()); //TODO handle nicely
    if fs::metadata(&config_path).is_err() {
        fs::create_dir_all(&config_path)?;
        let config_file = format!("{}/config.toml", config_path);
        let config = Config {
            notes_dir: String::from("wiki-o/notes"),
            file_format: String::from("md"),
        };
        let _config = toml::to_string(&config)?;
        fs::write(config_file, _config)?;
    }

    Ok(())
}

fn get_config() -> Result<Config> {
    let _config = fs::read_to_string(format!(
        "{}/.config/wiki-o/config.toml",
        home::home_dir().unwrap().display() //TODO handle nicely
    ))
    .expect("Unable to read file");
    return Ok(toml::from_str(&_config)?);
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InitialConfig {
    pub notes_abs_dir: String,
    pub file_format: String,
}

impl InitialConfig {
    pub fn init() -> Result<InitialConfig> {
        set_config()?;
        let config = get_config()?;
        let notes_abs_dir = format!(
            "{}/{}",
            home::home_dir().unwrap().display(), //TODO handle nicely
            config.notes_dir
        );

        let file_format = config.file_format;

        create_notes_dir(&notes_abs_dir)?;
        return Ok(InitialConfig {
            notes_abs_dir,
            file_format,
        });
    }
}

#[cfg(test)]
impl Default for InitialConfig {
    fn default() -> Self {
        let current_dir = std::env::current_dir().unwrap(); //TODO handle nicely
        let notes_abs_dir = format!("{}/test-dir/notes", current_dir.display());
        let file_format = "md".to_string();

        return InitialConfig {
            notes_abs_dir: notes_abs_dir,
            file_format: file_format,
        };
    }
}

#[derive(Deserialize, Serialize)]
struct Config {
    notes_dir: String,
    file_format: String,
}
