use std::{env::current_dir, fmt::Display, fs};

use crate::file;

use anyhow::Result;
use home::home_dir;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub notes_dir: String,
    pub metadata_dir: String,
    pub file_format: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "notes directory: {}\nmetadata directory: {}\nfile format: {}",
            self.notes_dir, self.metadata_dir, self.file_format
        )
    }
}

pub trait Environment {
    fn config(&self) -> Result<Config>;
    fn notes_abs_dir(&self) -> Result<String>;
    fn metadata_abs_dir(&self) -> Result<String>;
}
pub struct WContext {
    pub config_dir: String,
}

impl Environment for WContext {
    fn config(&self) -> Result<Config> {
        file::create_dir_if_not_exist(&format!(
            "{}/{}",
            home_dir().unwrap().display(),
            &self.config_dir
        ))?;

        let config = Config {
            notes_dir: "wiki-o/notes".to_string(),
            metadata_dir: "wiki-o/_metadata".to_string(),
            file_format: String::from("md"),
        };
        let config_file = format!("{}/config.toml", &self.config_dir);
        let _config = toml::to_string(&config)?;
        fs::write(config_file, _config)?;

        Ok(config)
    }

    fn notes_abs_dir(&self) -> Result<String> {
        self.config().and_then(|c: Config| {
            file::create_dir_if_not_exist(&format!(
                "{}/{}",
                home_dir().unwrap().display(),
                c.notes_dir
            ))
        })
    }

    fn metadata_abs_dir(&self) -> Result<String> {
        self.config().and_then(|c: Config| {
            file::create_dir_if_not_exist(&format!(
                "{}/{}",
                home_dir().unwrap().display(),
                c.metadata_dir
            ))
        })
    }
}

pub struct TestContext {
    pub config_dir: String,
}

impl Environment for TestContext {
    fn config(&self) -> Result<Config> {
        file::create_dir_if_not_exist(&format!(
            "{}/{}",
            current_dir().unwrap().display(),
            &self.config_dir
        ))?;

        let config = Config {
            notes_dir: "test-dir/notes".to_string(),
            metadata_dir: "test-dir/_metadata".to_string(),
            file_format: String::from("md"),
        };
        let config_file = format!("{}/config.toml", &self.config_dir);
        let _config = toml::to_string(&config)?;
        fs::write(config_file, _config)?;

        Ok(config)
    }

    fn notes_abs_dir(&self) -> Result<String> {
        self.config().and_then(|c: Config| {
            file::create_dir_if_not_exist(&format!(
                "{}/{}",
                current_dir().unwrap().display(),
                c.metadata_dir
            ))
        })
    }

    fn metadata_abs_dir(&self) -> Result<String> {
        self.config().and_then(|c: Config| {
            file::create_dir_if_not_exist(&format!(
                "{}/{}",
                current_dir().unwrap().display(),
                c.notes_dir
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use crate::env::{Config, Environment, TestContext};

    #[test]
    fn test_get_test_ctx() {
        let test_ctx = TestContext {
            config_dir: "test-dir/config".to_string(),
        };

        let expected = Config {
            notes_dir: "test-dir/notes".to_string(),
            metadata_dir: "test-dir/_metadata".to_string(),
            file_format: "md".to_string(),
        };

        assert_eq!(test_ctx.config().unwrap().file_format, expected.file_format);
        assert_eq!(test_ctx.config().unwrap().notes_dir, expected.notes_dir);

        super::file::delete_all_dirs(format!(
            "{}/{}",
            current_dir().unwrap().display(),
            "test-dir"
        ))
        .unwrap();
    }

    // #[test]
    // fn test_get_notes_abs_dir() {
    //     let test_ctx = TestContext {
    //         config_dir: "test-dir/config".to_string(),
    //     };

    //     let config = test_ctx.config().unwrap();

    //     let expected = format!("{}/{}", current_dir().unwrap().display(), config.notes_dir);

    //     assert_eq!(test_ctx.notes_abs_dir().unwrap(), expected);

    //     super::file::delete_all_dirs(format!(
    //         "{}/{}",
    //         current_dir().unwrap().display(),
    //         "test-dir".to_string()
    //     ))
    //     .unwrap();
    // }
}
