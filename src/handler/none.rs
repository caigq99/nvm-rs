use std::process::Command;
pub fn handle_none() {
    Command::new("nvm-rs").args(["--help"]).spawn().unwrap();
}
