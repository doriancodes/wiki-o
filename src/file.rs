use anyhow::Result;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs;
use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use crate::logging::deleted;
use crate::logging::added;

#[derive(Debug, Deserialize, Serialize)]
pub struct WikioFile {
    pub file: String,
    pub content: String,
    pub file_name: String,
}

pub fn read_from_file(file_path: &String) -> Result<String> {
    let content = std::fs::read_to_string(file_path)?;
    Ok(content)
}

pub fn write_to_file(file_name: String, file_path: String, content: String) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .append(true)
        .open(file_path)?;

    file.seek(SeekFrom::Start(0))?;
    file.write_all(content.as_bytes())?;

    added(content.trim().to_string(), file_name.to_string());

    Ok(())
}

pub fn delete_file(file: String) -> Result<()> {
    fs::remove_file(&file)?;
    deleted(true, file);
    Ok(())
}

pub fn delete_all_dirs(dir: String) -> Result<()> {
    let deleted_dir = std::fs::remove_dir_all(&dir);
    if deleted_dir.is_ok() {
        deleted(false, dir);
    }
    Ok(())
}

pub fn read_all_files_in_dir(dir: String) -> Result<Vec<WikioFile>> {
    let paths = fs::read_dir(dir)?;

    let mut files: Vec<WikioFile> = vec![];

    for path in paths {
        let path = path?;
        let file_i = path.file_name().to_str().get_or_insert("").to_string();
        let path_i = path.path().display().to_string();
        let content = read_from_file(&path_i)?;

        files.push(WikioFile {
            file: path_i.clone(),
            content: content.clone(),
            file_name: file_i.clone(),
        });


    }

    return Ok(files);
}

pub fn create_dir_if_not_exist(dir: &String) -> Result<String> {
    if fs::metadata(&dir).is_err() {
        fs::create_dir_all(&dir)?;
        return Ok(dir.clone());
    }

    Ok(dir.clone())
}
