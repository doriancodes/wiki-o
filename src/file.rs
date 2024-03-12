use anyhow::Result;
use std::fs;
use std::fs::OpenOptions;
use std::fs::ReadDir;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

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

    println!("Added {} to {}", content.trim(), file_name);

    Ok(())
}

pub fn delete_file(file: String) -> Result<()> {
    fs::remove_file(&file)?;
    println!("Deleted {}", file);
    Ok(())
}

pub fn delete_all_dirs(dir: String) -> Result<()> {
    let deleted = std::fs::remove_dir_all(&dir);
    if deleted.is_ok() {
        println!("Deleted directory: {}", dir);
    }
    Ok(())
}

pub fn read_all_files_in_dir(dir: String) -> Result<ReadDir> {
    Ok(fs::read_dir(dir)?)
}

pub fn create_dir_if_not_exist(dir: &String) -> Result<String> {
    if fs::metadata(&dir).is_err() {
        fs::create_dir_all(&dir)?;
        return Ok(dir.clone());
    }

    Ok(dir.clone())
}
