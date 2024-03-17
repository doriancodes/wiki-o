use anyhow::Result;

use crate::{
    file::{self, WikioFile},
    logging::{header, text},
    src_engine::{self, Engine, ReadOperation, WDocument, WriteOperation},
};

pub fn add(
    content: &String,
    file_name: &String,
    notes_dir: &String,
    file_format: &String,
    metadara_dir: &String,
) -> Result<()> {
    let content_f = format!("{}\n\n", content);

    let file_path = format!("{}/{}.{}", notes_dir, file_name, file_format);

    file::write_to_file(file_name.clone(), file_path.clone(), content_f.clone())?;

    let eng = Engine::new(metadara_dir)?;
    let mut writer = WriteOperation { engine: eng };
    writer.build_index(vec![WDocument {
        title: file_name.clone(),
        body: content.clone(),
    }])?;

    Ok(())
}

pub fn show(file_name: &String, notes_dir: &str) -> Result<Vec<WikioFile>> {
    let files = file::read_all_files_in_dir(notes_dir.to_owned())?;
    files
        .iter()
        .filter(|f| f.file_name.contains(file_name))
        .collect::<Vec<&WikioFile>>()
        .iter()
        .for_each(|f| {
            header("File:".to_string(), f.file_name.clone());
            text(f.content.clone().as_str());
        });
    Ok(files)
}

pub fn list(is_short: bool, notes_dir: &str) -> Result<Vec<WikioFile>> {
    let files = file::read_all_files_in_dir(notes_dir.to_owned())?;

    files.iter().for_each(|f: &WikioFile| {
        header("File:".to_string(), f.file_name.clone());
        if !is_short {
            text(f.content.clone().as_str());
        }
    });

    Ok(files)
}

pub fn search(search_str: &str, metadara_dir: &String) -> Result<()> {
    let eng = src_engine::Engine::new(metadara_dir)?;

    let reader = ReadOperation { engine: eng };

    println!("Searching for: {}", search_str);

    reader.search(search_str)?;

    Ok(())
}

pub fn delete(
    notes_abs_dir: &String,
    metadara_dir: &String,
    file_name: &String,
    file_format: &String,
) -> Result<()> {
    let eng = src_engine::Engine::new(metadara_dir)?;
    let mut writer = WriteOperation { engine: eng };

    writer.remove_document_index(&file_name)?;
    let file = format!("{}/{}.{}", notes_abs_dir, file_name, file_format);

    file::delete_file(file.clone())?;
    Ok(())
}

pub fn purge(notes_abs_dir: &str, metadara_dir: &String) -> Result<()> {
    let eng = src_engine::Engine::new(metadara_dir)?;
    let mut writer = WriteOperation { engine: eng };

    file::delete_all_dirs(notes_abs_dir.to_owned())?;
    writer.remove_all_documents_index()?;
    //todo purge metadata
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::action::{add, delete, list, purge};
    use crate::env::{Environment, TestContext};
    use crate::file;

    use std::fs;

    fn setup() -> (String, String, String, String, String) {
        let test_ctx: TestContext = TestContext {
            config_dir: "test-dir/config".to_string(),
        };
        let config = test_ctx.config().unwrap();

        let content = "test content".to_string();
        let file_name = "test".to_string();

        let notes_dir = test_ctx.notes_abs_dir().unwrap();
        let metadata_dir = test_ctx.metadata_abs_dir().unwrap();

        (
            content,
            file_name,
            notes_dir,
            config.file_format,
            metadata_dir,
        )
    }

    fn teardown() {
        fs::remove_dir_all("test-dir").unwrap();
    }

    #[test]
    fn test_add() {
        let (content, file_name, notes_dir, file_format, metadata_dir) = setup();

        add(
            &content,
            &file_name,
            &notes_dir,
            &file_format,
            &metadata_dir,
        )
        .unwrap();

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
        let (content, file_name, notes_dir, file_format, metadata_dir) = setup();

        add(
            &content,
            &file_name,
            &notes_dir,
            &file_format,
            &metadata_dir,
        )
        .unwrap();

        let files = list(false, &notes_dir);

        assert_eq!(
            files.unwrap()[0].file,
            format!("{}/{}.{}", &notes_dir, &file_name, &file_format)
        );

        teardown();
    }

    #[test]
    fn test_delete() {
        let (content, file_name, notes_dir, file_format, metadata_dir) = setup();

        add(
            &content,
            &file_name,
            &notes_dir,
            &file_format,
            &metadata_dir,
        )
        .unwrap();
        delete(&notes_dir, &metadata_dir, &file_name, &file_format).unwrap();

        assert!(
            fs::read_to_string(format!("{}/{}.{}", &notes_dir, &file_name, &file_format)).is_err()
        );

        teardown();
    }

    #[test]
    fn test_purge() {
        let (content, file_name, notes_dir, file_format, metadata_dir) = setup();

        add(
            &content,
            &file_name,
            &notes_dir,
            &file_format,
            &metadata_dir,
        )
        .unwrap();
        purge(&notes_dir, &metadata_dir).unwrap();

        assert!(file::read_all_files_in_dir(notes_dir.clone()).is_err());

        teardown();
    }
}
