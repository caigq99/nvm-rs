mod cli;
mod commands;
mod handler;

use clap::Parser;
use cli::Cli;
use commands::Commands;
fn main() {
    let cli_config = Cli::parse();
    match cli_config.command {
        None => {
            handler::handle_none();
        }
        Some(Commands::Arch) => handler::handle_arch(),
        Some(Commands::Current) => {
            handler::handle_current();
        }
        Some(Commands::Install) => {
            handler::handle_install();
        }
        Some(Commands::Uninstall) => {
            handler::handle_uninstall();
        }
        Some(Commands::List) => {
            handler::handle_list();
        }
        Some(Commands::Proxy { url }) => {
            handler::handle_proxy();
        }
        Some(Commands::Use { version }) => {
            handler::handle_use_version();
        }
    }
}
