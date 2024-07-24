use anyhow::Result;
use dirs;
use std::fs;
use std::path::{Path, PathBuf};

pub fn config_path() -> String {
    dirs::home_dir()
        .unwrap()
        .join(".config")
        .join("nvm.toml")
        .to_str()
        .unwrap()
        .to_string()
}

pub fn get_default_node_path() -> PathBuf {
    dirs::home_dir().unwrap().join(PathBuf::from(".nodefile"))
}

pub fn config_file_exists(file_path: String) -> bool {
    Path::new(&file_path).exists()
}

pub fn config_file_create(file_path: String) -> Result<fs::File, std::io::Error> {
    fs::File::create(file_path)
}

pub fn config_file_read(file_path: String) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

pub fn config_file_write(file_path: String, content: String) -> Result<(), std::io::Error> {
    fs::write(file_path, content)
}
