use crate::config;

pub struct Context {
    pub initial_config: config::InitialConfig,
    pub file_buffer: Option<BufWriter<File>>
}

impl Context {
    pub fn with_buffer(file_path: String) -> Context {
        let file = File::create(file_path).unwrap(); //TODO handle nicely
        let file_buffer = BufWriter::new(file);
        Context {
            initial_config: config::InitialConfig::init(),
            file_buffer: Some(file_buffer)
        }
    }

    pub fn without_buffer() -> Context {
        Context {
            initial_config: config::InitialConfig::init(),
            file_buffer: None
        }
    }
}