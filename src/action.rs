use anyhow::Result;
use serde_derive::Deserialize;
use serde_derive::Serialize;


use crate::file;

pub fn add(
    content: &String,
    file_name: &String,
    notes_dir: &String,
    config_dir: &String,
    file_format: &String,
) -> Result<()> {
    let content_f = format!("{}\n\n", content);

    let file_path = format!("{}/{}.{}", notes_dir, file_name, file_format);

    file::write_to_csv(&file_name, &file_path, config_dir)?;

    file::write_to_file(file_name.clone(), file_path.clone(), content_f.clone())?;

    Ok(())
}

pub fn list(is_short: bool, notes_dir: &String) -> Result<Vec<WikioFile>> {
    let paths = file::read_all_files_in_dir(notes_dir.clone())?;
    let mut files: Vec<WikioFile> = vec![];

    for path in paths {
        let path_i = path?.path().display().to_string();
        let content = file::read_from_file(&path_i)?;

        files.push(WikioFile {
            file: path_i.clone(),
            content: content.clone(),
        });

        println!("File: {}", path_i);
        if !is_short {
            print!("\n{}\n", content);
        }
    }

    return Ok(files);
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WikioFile {
    pub file: String,
    pub content: String,
}

pub fn delete(notes_abs_dir: &String, config_abs_dir: &String, file_name: &String) -> Result<()> {
    let file = format!("{}/{}", notes_abs_dir, file_name);
    let config_file = format!("{}/_metadata.csv", config_abs_dir);

    file::delete_file(file)?;
    //file::delete_file(config_file)?; //TODO decide what to do with the metadata
    Ok(())
}

pub fn purge(notes_abs_dir: &String, config_abs_dir: &String) -> Result<()> {
    file::delete_all_dirs(notes_abs_dir.clone())?;
    file::delete_all_dirs(config_abs_dir.clone())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::action::{add, delete, list, purge};
    use crate::config;
    use std::fs;

    fn setup() -> (String, String, String, String, String) {
        let initial_config = config::InitialConfig::default();

        let content = "test content".to_string();
        let file_name = "test".to_string();

        fs::create_dir_all(&initial_config.notes_abs_dir).unwrap();
        fs::create_dir_all(&initial_config.config_abs_dir).unwrap();

        return (
            content,
            file_name,
            initial_config.notes_abs_dir,
            initial_config.config_abs_dir,
            initial_config.file_format,
        );
    }

    fn teardown() {
        let initial_config = config::InitialConfig::default();

        println!("{:#?}", initial_config);

        fs::remove_dir_all("test-dir").unwrap();
    }

    #[test]
    fn test_add() {
        let (content, file_name, notes_dir, config_dir, file_format) = setup();

        add(&content, &file_name, &notes_dir, &config_dir, &file_format).unwrap();

        assert_eq!(
            fs::read_to_string(format!("{}/{}.{}", &notes_dir, &file_name, &file_format))
                .unwrap()
                .trim(),
            content
        );

        teardown();
    }

    #[test]
    fn test_list() {
        let (content, file_name, notes_dir, config_dir, file_format) = setup();

        add(&content, &file_name, &notes_dir, &config_dir, &file_format).unwrap();

        let files = list(false, &notes_dir);

        assert_eq!(
            files.unwrap()[0].file,
            format!("{}/{}.{}", &notes_dir, &file_name, &file_format)
        );

        teardown();
    }

    // #[test]
    // fn test_delete() {
    //     let (content, file_name, notes_dir, config_dir, file_format) = setup();

    //     add(&content, &file_name, &notes_dir, &config_dir, &file_format).unwrap();
    //     purge(&notes_dir, &config_dir).unwrap();

    //     let result = purge(&notes_dir, &config_dir);

    //     assert!(result.is_err());

    //     teardown();
    // }
}
