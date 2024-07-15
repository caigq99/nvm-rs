mod config;
mod platform;

pub use config::{
    config_file_create, config_file_exists, config_file_read, config_file_write, config_path,
};

pub use platform::{is_supported_system, is_windows, SystemInfo};
