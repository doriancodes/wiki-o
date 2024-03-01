use std::{error::Error, fs::OpenOptions, io::Seek};

use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

pub fn write_to_csv(file_name: String, config_dir: String) -> Result<(), Box<dyn Error>> {
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
    wtr.serialize([MetadataRow {
        id: uuid,
        file_name: file_name.clone(),
    }])?;

    wtr.flush()?;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct MetadataRow {
    id: String,
    file_name: String,
}
