use dirs;
use std::path::PathBuf;

pub fn get_default_node_path() -> PathBuf {
    dirs::home_dir().unwrap().join(PathBuf::from(".nodefile"))
}
