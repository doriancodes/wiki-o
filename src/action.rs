use anyhow::Result;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::file;

pub fn add(
    content: &String,
    file_name: &String,
    notes_dir: &String,
    file_format: &String,
) -> Result<()> {
    let content_f = format!("{}\n\n", content);

    let file_path = format!("{}/{}.{}", notes_dir, file_name, file_format);

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

pub fn delete(notes_abs_dir: &String, file_name: &String, file_format: &String) -> Result<()> {
    let file = format!("{}/{}.{}", notes_abs_dir, file_name, file_format);

    file::delete_file(file)?;
    Ok(())
}

pub fn purge(notes_abs_dir: &String, config_dir: String) -> Result<()> {
    file::delete_all_dirs(notes_abs_dir.clone())?;
    file::delete_all_dirs(config_dir.clone())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::action::{add, delete, list, purge};
    use crate::context;
    use crate::file;

    use std::env::current_dir;
    use std::fs;

    fn setup() -> (String, String, String, String, String) {
        let context = context::Context::default();

        let content = "test content".to_string();
        let file_name = "test".to_string();

        fs::create_dir_all(&context.initial_config.notes_abs_dir).unwrap();

        let current_dir = current_dir().unwrap().display().to_string();

        let notes_dir = file::format_file_name(&current_dir, &context.initial_config.notes_abs_dir);

        (
            content,
            file_name,
            notes_dir,
            context.initial_config.file_format,
            context.config_path,
        )
    }

    fn teardown() {
        fs::remove_dir_all("test-dir").unwrap();
    }

    #[test]
    fn test_add() {
        let (content, file_name, notes_dir, file_format, _) = setup();

        add(&content, &file_name, &notes_dir, &file_format).unwrap();

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
        let (content, file_name, notes_dir, file_format, _) = setup();

        add(&content, &file_name, &notes_dir, &file_format).unwrap();

        let files = list(false, &notes_dir);

        assert_eq!(
            files.unwrap()[0].file,
            format!("{}/{}.{}", &notes_dir, &file_name, &file_format)
        );

        teardown();
    }

    #[test]
    fn test_delete() {
        let (content, file_name, notes_dir, file_format, _) = setup();

        add(&content, &file_name, &notes_dir, &file_format).unwrap();
        delete(&notes_dir, &file_name, &file_format).unwrap();

        assert!(
            fs::read_to_string(format!("{}/{}.{}", &notes_dir, &file_name, &file_format)).is_err()
        );

        teardown();
    }

    #[test]
    fn test_purge() {
        let (content, file_name, notes_dir, file_format, config_path) = setup();

        add(&content, &file_name, &notes_dir, &file_format).unwrap();
        purge(&notes_dir, config_path.clone()).unwrap();

        assert!(file::read_all_files_in_dir(notes_dir.clone()).is_err());
        assert!(file::read_all_files_in_dir(config_path).is_err());

        teardown();
    }
}
