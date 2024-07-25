mod cli;
mod commands;
mod handler;
mod utils;

use clap::Parser;
use cli::Cli;
use commands::Commands;

#[tokio::main]
async fn main() {
    let cli_config = Cli::parse();
    match cli_config.command {
        None => {
            handler::handle_none();
        }
        Some(Commands::Arch) => handler::handle_arch(),
        Some(Commands::Current) => {
            handler::handle_current();
        }
        Some(Commands::Install { version }) => {
            handler::handle_install(version);
        }
        Some(Commands::Uninstall) => {
            handler::handle_uninstall();
        }
        Some(Commands::List) => {
            handler::handle_list();
        }
        Some(Commands::Use { version: _ }) => {
            handler::handle_use_version();
        }
    }
}
