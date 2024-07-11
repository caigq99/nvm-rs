use crate::commands;
use clap::Parser;
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<commands::Commands>,
}
