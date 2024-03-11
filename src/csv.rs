use std::{fs::OpenOptions, io::Seek};

use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

pub fn write_to_csv(file_name: &String, file_path: String, config_dir: &String) -> Result<()> {
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
