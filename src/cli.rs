use clap::Parser;
use clap::Subcommand;

use crate::subcommands::init::InitArgs;

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project inside of the current directory
    Init(InitArgs),
}

#[derive(Parser)]
#[command(name = "ignitec")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
