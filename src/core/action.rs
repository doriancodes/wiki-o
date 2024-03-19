use anyhow::Result;

use crate::io::env;
use crate::io::file;
use crate::io::file::WikioFile;
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

pub fn delete(file_name: &String, file_format: &String, env: &env::WEnv) -> Result<()> {
    let file = format!("{}/{}.{}", env.notes_abs_dir(), file_name, file_format);

    file::delete_file(file.clone())?;
    Ok(())
}

pub fn purge(env: &env::WEnv) -> Result<()> {
    file::delete_all_dirs(env.notes_abs_dir())?;

    Ok(())
}
