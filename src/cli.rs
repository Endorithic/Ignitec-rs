use clap::Parser;
use clap::Subcommand;

use crate::clap_styles::CLAP_STYLE;

use crate::subcommands::build::BuildArgs;
use crate::subcommands::init::InitArgs;

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project inside of the current directory
    Init(InitArgs),
    /// Build the current project
    Build(BuildArgs),
}

#[derive(Parser)]
#[command(name = "ignitec", styles = CLAP_STYLE)]
pub struct Cli {
    #[arg(short, long, default_value_t = false, global = true)]
    /// Whether all log messages should also output to a log file
    pub log: bool,

    #[command(subcommand)]
    pub command: Commands,
}
