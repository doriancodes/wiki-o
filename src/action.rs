use anyhow::Result;

use crate::file::{self, WikioFile};

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
    file::read_all_files_in_dir(notes_dir.clone(), !is_short)
}

pub fn delete(notes_abs_dir: &String, file_name: &String, file_format: &String) -> Result<()> {
    let file = format!("{}/{}.{}", notes_abs_dir, file_name, file_format);

    file::delete_file(file.clone())?;
    Ok(())
}

pub fn purge(notes_abs_dir: &String) -> Result<()> {
    file::delete_all_dirs(notes_abs_dir.clone())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::action::{add, delete, list, purge};
    use crate::env::{Environment, TestContext};
    use crate::file;

    use std::fs;

    fn setup() -> (String, String, String, String) {
        let test_ctx: TestContext = TestContext {
            config_dir: "test-dir/config".to_string(),
        };
        let config = test_ctx.config().unwrap();

        let content = "test content".to_string();
        let file_name = "test".to_string();

        let notes_dir = test_ctx.notes_abs_dir().unwrap();

        (content, file_name, notes_dir, config.file_format)
    }

    fn teardown() {
        fs::remove_dir_all("test-dir").unwrap();
    }

    #[test]
    fn test_add() {
        let (content, file_name, notes_dir, file_format) = setup();

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
        let (content, file_name, notes_dir, file_format) = setup();

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
        let (content, file_name, notes_dir, file_format) = setup();

        add(&content, &file_name, &notes_dir, &file_format).unwrap();
        delete(&notes_dir, &file_name, &file_format).unwrap();

        assert!(
            fs::read_to_string(format!("{}/{}.{}", &notes_dir, &file_name, &file_format)).is_err()
        );

        teardown();
    }

    #[test]
    fn test_purge() {
        let (content, file_name, notes_dir, file_format) = setup();

        add(&content, &file_name, &notes_dir, &file_format).unwrap();
        purge(&notes_dir).unwrap();

        assert!(file::read_all_files_in_dir(notes_dir.clone(), false).is_err());

        teardown();
    }
}
