use anyhow::Result;

use crate::io::env;
use crate::io::file;
use crate::io::file::WikioFile;
use crate::io::src_engine::*;
use crate::logging::logger::*;

pub fn add(
    content: &String,
    file_name: &String,
    file_format: &String,
    env: &env::WEnv,
) -> Result<WikioFile> {
    let content_f = format!("{}\n\n", content);

    let notes_dir = env.notes_abs_dir();

    let file_path = format!("{}/{}.{}", notes_dir, file_name, file_format);

    println!("File path: {}", file_path);

    let wfile = WikioFile {
        file_name: file_name.clone(),
        content: content_f,
        file: file_path,
    };

    file::write_to_file(
        wfile.file_name.clone(),
        wfile.file.clone(),
        wfile.content.clone(),
    )?;

    let eng = Engine::new(&env.metadata_abs_dir())?;
    let mut writer = WriteOperation { engine: eng };
    writer.build_index(vec![WDocument {
        title: file_name.clone(),
        body: content.clone(),
    }])?;

    Ok(wfile)
}

pub fn show(file_name: &String, is_complete: &bool, env: &env::WEnv) -> Result<Vec<WikioFile>> {
    let files = file::read_all_files_in_dir(env.notes_abs_dir().to_owned())?;
    files
        .iter()
        .filter(|f| f.file_name.contains(file_name))
        .collect::<Vec<&WikioFile>>()
        .iter()
        .for_each(|f| {
            if *is_complete {
                header("File:".to_string(), f.file_name.clone());
            }
            text(f.content.to_owned().as_str());
        });
    Ok(files)
}

pub fn list(is_short: bool, env: &env::WEnv) -> Result<Vec<WikioFile>> {
    let files = file::read_all_files_in_dir(env.notes_abs_dir().to_owned())?;

    files.iter().for_each(|f: &WikioFile| {
        header("File:".to_string(), f.file_name.clone());
        if !is_short {
            text(f.content.clone().as_str());
        }
    });

    Ok(files)
}

pub fn search(search_str: &str, env: &env::WEnv) -> Result<()> {
    let eng = Engine::new(&env.metadata_abs_dir())?;

    let reader = ReadOperation { engine: eng };

    reader.search(search_str)?;

    Ok(())
}

pub fn delete(file_name: &String, file_format: &String, env: &env::WEnv) -> Result<()> {
    let eng = Engine::new(&env.metadata_abs_dir())?;
    let mut writer = WriteOperation { engine: eng };

    writer.remove_document_index(file_name)?;
    let file = format!("{}/{}.{}", env.notes_abs_dir(), file_name, file_format);

    file::delete_file(file.clone())?;
    Ok(())
}

pub fn purge(env: &env::WEnv) -> Result<()> {
    let eng = Engine::new(&env.metadata_abs_dir())?;
    let mut writer = WriteOperation { engine: eng };

    file::delete_all_dirs(env.notes_abs_dir())?;
    writer.remove_all_documents_index()?;
    //todo purge metadata
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::core::action::{add, delete, list, purge};
    use crate::io;
    use crate::io::env::WEnv;

    use std::fs;

    fn setup() -> (String, String, String, String, String) {
        let init_dir = io::env::ContextWriter { env: WEnv::Test };

        init_dir.init().unwrap();

        let env = WEnv::Test;
        let config = env.config();

        let content = "test content".to_string();
        let file_name = "test".to_string();

        let notes_dir = env.notes_abs_dir();
        let metadata_dir = env.metadata_abs_dir();

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
        let (content, file_name, notes_dir, file_format, _) = setup();

        add(&content, &file_name, &file_format, &WEnv::Test).unwrap();

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

        add(&content, &file_name, &file_format, &WEnv::Test).unwrap();

        let files = list(false, &WEnv::Test);

        assert_eq!(
            files.unwrap()[0].file,
            format!("{}/{}.{}", &notes_dir, &file_name, &file_format)
        );

        teardown();
    }

    #[test]
    fn test_delete() {
        let (content, file_name, notes_dir, file_format, _) = setup();

        add(&content, &file_name, &file_format, &WEnv::Test).unwrap();
        delete(&file_name, &file_format, &WEnv::Test).unwrap();

        assert!(
            fs::read_to_string(format!("{}/{}.{}", &notes_dir, &file_name, &file_format)).is_err()
        );

        teardown();
    }

    #[test]
    fn test_purge() {
        let (content, file_name, notes_dir, file_format, _) = setup();

        add(&content, &file_name, &file_format, &WEnv::Test).unwrap();
        purge(&WEnv::Test).unwrap();

        assert!(io::file::read_all_files_in_dir(notes_dir.clone()).is_err());

        teardown();
    }
}
