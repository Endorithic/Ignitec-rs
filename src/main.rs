mod cli;
mod logger;
mod subcommands;

use clap::Parser;

use cli::Cli;
use cli::Commands;
use logger::Logger;
use subcommands::init;

fn main() {
    let cli: Cli = Cli::parse();
    let mut logger: Logger = Logger::new();

    if let Err(error) = match cli.command {
        Commands::Init(args) => init(&mut logger, &args),
    } {
        error!(logger, "{:#}", error);
        std::process::exit(1);
    }
}
