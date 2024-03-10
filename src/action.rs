use core::result::Result::Ok;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs;
use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use crate::csv;

pub fn add(
    content: &String,
    file_name: &String,
    notes_dir: &String,
    config_dir: &String,
    file_format: &String,
) {
    let content_f = format!("{}\n\n", content);

    let file_path = format!("{}/{}.{}", notes_dir, file_name, file_format);

    csv::write_to_csv(&file_name, file_path.clone(), config_dir).unwrap();

    let mut note = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .create(true)
        .open(file_path)
        .unwrap();
    note.seek(SeekFrom::Start(0)).unwrap();
    note.write_all(content_f.as_bytes()).unwrap();

    println!("Added {} to {}", content, file_name);
}

pub fn list(is_short: bool, notes_dir: &String) -> Vec<WikioFile> {
    let paths = fs::read_dir(&notes_dir).unwrap();
    let mut files: Vec<WikioFile> = vec![];

    for path in paths {
        let path_i = path.unwrap().path().display().to_string();
        let content = fs::read_to_string(&path_i)
            .expect("unable to read file")
            .to_string();

        files.push(WikioFile {
            file: path_i.clone(),
            content: content.clone(),
        });

        println!("File: {}", path_i);
        if !is_short {
            print!("\n{}\n", content);
        }
    }

    return files;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WikioFile {
    pub file: String,
    pub content: String,
}

fn format_for_delete(is_err: bool, dir: String, e: Option<std::io::Error>) -> String {
    let mut res = String::new();

    if is_err {
        res.push_str(
            format!(
                "Failed to delete {}, error {}\n",
                dir,
                e.unwrap().to_string()
            )
            .as_str(),
        );
    } else {
        res.push_str(format!("Deleted directory: {}\n", dir).as_str());
    }

    return res;
}

pub fn delete(notes_abs_dir: &String, config_abs_dir: &String) {
    let mut message = String::new();
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    match std::fs::remove_dir_all(notes_abs_dir) {
        Ok(_) => message.push_str(&format_for_delete(false, notes_abs_dir.clone(), None)),
        Err(e) => message.push_str(&format_for_delete(true, notes_abs_dir.clone(), Some(e))),
    };

    match std::fs::remove_dir_all(config_abs_dir) {
        Ok(_) => message.push_str(&format_for_delete(false, config_abs_dir.clone(), None)),
        Err(e) => message.push_str(&format_for_delete(true, config_abs_dir.clone(), Some(e))),
    };

    writeln!(handle, "{}", message).unwrap();

    // println!("{}",message);
}

#[cfg(test)]
mod tests {
    use crate::action::{add, delete, list};
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

        add(&content, &file_name, &notes_dir, &config_dir, &file_format);

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

        add(&content, &file_name, &notes_dir, &config_dir, &file_format);

        let files = list(false, &notes_dir);

        assert_eq!(
            files[0].file,
            format!("{}/{}.{}", &notes_dir, &file_name, &file_format)
        );
    }

    #[test]
    fn test_delete() {
        let (content, file_name, notes_dir, config_dir, file_format) = setup();

        add(&content, &file_name, &notes_dir, &config_dir, &file_format);
        delete(&notes_dir, &config_dir);

        let result = delete(&notes_dir, &config_dir);

        // let expected: Err = Err(std::io::Error::from(std::io::ErrorKind::NotFound));

        // assert_eq!(fs::read_dir(&notes_dir).unwrap().count(), 0);
        // assert_eq!(fs::read_dir(&config_dir).unwrap().count(), 0);
    }
}
