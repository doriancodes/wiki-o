use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::fs::ReadDir;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use uuid::Uuid;

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
    std::fs::remove_file(&file)?;
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
    Ok(std::fs::read_dir(dir)?)
}

pub fn write_to_csv(file_name: &String, file_path: &String, config_dir: &String) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .append(true)
        .open(format!("{}/_metadata.csv", config_dir))?;

    let needs_headers = file.seek(std::io::SeekFrom::End(0))? == 0;

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(needs_headers)
        .from_writer(file);

    let uuid = Uuid::new_v4().to_string();
    let date_added = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    wtr.serialize([MetadataRow {
        id: uuid,
        file_name: file_name.clone(),
        file_path: file_path.clone(),
        date_added: date_added,
    }])?;

    wtr.flush()?;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct MetadataRow {
    id: String,
    file_name: String,
    file_path: String,
    date_added: String,
}
