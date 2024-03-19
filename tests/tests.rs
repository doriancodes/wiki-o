extern crate wiki_o;

use std::fs;

use wiki_o::core::action::{add, delete, list, purge};
use wiki_o::io;
use wiki_o::io::env::WEnv;

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
fn integration_test() {
    let (content, file_name, notes_dir, file_format, _) = setup();
    let expected_res = format!("{}/{}.{}", &notes_dir, &file_name, &file_format);

    add(&content, &file_name, &file_format, &WEnv::Test).unwrap();

    assert_eq!(fs::read_to_string(&expected_res).unwrap().trim(), content);

    let files = list(false, &WEnv::Test);

    assert_eq!(files.unwrap()[0].file, expected_res.clone());

    delete(&file_name, &file_format, &WEnv::Test).unwrap();

    assert!(fs::read_to_string(format!("{}/{}.{}", &notes_dir, &file_name, &file_format)).is_err());

    add(&content, &file_name, &file_format, &WEnv::Test).unwrap();
    purge(&WEnv::Test).unwrap();

    assert!(io::file::read_all_files_in_dir(notes_dir.clone()).is_err());

    teardown();
}
