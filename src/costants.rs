use std::{
    env::current_dir,
    sync::{Mutex, OnceLock},
};

use home::home_dir;

fn get_config_dir() -> &'static Mutex<String> {
    static CONFIG_FILE: OnceLock<Mutex<String>> = OnceLock::new();
    CONFIG_FILE
        .get_or_init(|| Mutex::new(format!("{}/.config/wiki-o", home_dir().unwrap().display())))
}

fn get_test_config_dir() -> &'static Mutex<String> {
    static CONFIG_FILE: OnceLock<Mutex<String>> = OnceLock::new();
    CONFIG_FILE.get_or_init(|| {
        Mutex::new(format!(
            "{}/test-dir/config",
            current_dir().unwrap().display()
        ))
    })
}

pub fn get_env_config(prod: bool) -> (String, String) {
    let env_config: &Mutex<String> = if prod {
        get_config_dir()
    } else {
        get_test_config_dir()
    };
    let config_path = env_config.lock().unwrap().as_str().to_string();
    (config_path.clone(), "config.toml".to_string())
}
