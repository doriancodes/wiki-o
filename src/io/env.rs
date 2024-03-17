use std::{env::current_dir, fmt::Display};

use crate::io::file;

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

pub enum WEnv {
    Prod,
    Test,
}

impl WEnv {
    fn config_base_dir(&self) -> String {
        match self {
            Self::Prod => home_dir().unwrap().display().to_string(),
            Self::Test => current_dir().unwrap().display().to_string(),
        }
    }

    fn wikio_base_dir(&self) -> String {
        match self {
            Self::Prod => home_dir().unwrap().display().to_string(),
            Self::Test => current_dir().unwrap().display().to_string(),
        }
    }

    fn config_dir(&self) -> String {
        match self {
            Self::Prod => ".config/wiki-o".to_string(),
            Self::Test => "test-dir/config".to_string(),
        }
    }

    pub fn config(&self) -> Config {
        match self {
            Self::Prod => Config {
                notes_dir: "wiki-o/notes".to_string(),
                metadata_dir: "wiki-o/_metadata".to_string(),
                file_format: String::from("md"),
            },
            Self::Test => Config {
                notes_dir: "test-dir/notes".to_string(),
                metadata_dir: "test-dir/_metadata".to_string(),
                file_format: String::from("md"),
            },
        }
    }

    pub fn notes_abs_dir(&self) -> String {
        format!("{}/{}", self.wikio_base_dir(), self.config().notes_dir)
    }

    pub fn metadata_abs_dir(&self) -> String {
        format!("{}/{}", self.wikio_base_dir(), self.config().metadata_dir)
    }
}

pub struct ContextWriter {
    pub env: WEnv,
}

impl ContextWriter {
    pub fn init(&self) -> Result<()> {
        self.create_config_dir()?;
        self.create_dir_if_not_exist()?;
        Ok(())
    }

    fn create_config_dir(&self) -> Result<()> {
        file::create_dir_if_not_exist(&format!(
            "{}/{}",
            self.env.config_base_dir(),
            self.env.config_dir()
        ))?;
        Ok(())
    }

    fn create_dir_if_not_exist(&self) -> Result<()> {
        file::create_dir_if_not_exist(&self.env.notes_abs_dir())?;
        file::create_dir_if_not_exist(&self.env.metadata_abs_dir())?;
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use std::env::current_dir;

//     use crate::io::env::Config;

//     #[test]
//     fn test_get_test_ctx() {
//         let test_ctx = TestContext {
//             config_dir: "test-dir/config".to_string(),
//         };

//         let expected = Config {
//             notes_dir: "test-dir/notes".to_string(),
//             metadata_dir: "test-dir/_metadata".to_string(),
//             file_format: "md".to_string(),
//         };

//         assert_eq!(test_ctx.config().unwrap().file_format, expected.file_format);
//         assert_eq!(test_ctx.config().unwrap().notes_dir, expected.notes_dir);

//         super::file::delete_all_dirs(format!(
//             "{}/{}",
//             current_dir().unwrap().display(),
//             "test-dir"
//         ))
//         .unwrap();
//     }

//     // #[test]
//     // fn test_get_notes_abs_dir() {
//     //     let test_ctx = TestContext {
//     //         config_dir: "test-dir/config".to_string(),
//     //     };

//     //     let config = test_ctx.config().unwrap();

//     //     let expected = format!("{}/{}", current_dir().unwrap().display(), config.notes_dir);

//     //     assert_eq!(test_ctx.notes_abs_dir().unwrap(), expected);

//     //     super::file::delete_all_dirs(format!(
//     //         "{}/{}",
//     //         current_dir().unwrap().display(),
//     //         "test-dir".to_string()
//     //     ))
//     //     .unwrap();
//     // }
// }
