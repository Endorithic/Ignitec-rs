mod cli;
mod core;
mod logger;
mod subcommands;

use std::path;

use clap::Parser;

use cli::Cli;
use cli::Commands;
use core::output;
use logger::Logger;
use subcommands::init;

fn main() {
    let cli: Cli = Cli::parse();

    let mut logger: Logger = if cli.log {
        let log_directory: path::PathBuf = match output::log_directory() {
            Ok(path) => path,
            Err(error) => {
                eprintln!("Error: Failed to get log directory: {error}");
                return;
            }
        };

        let log_name: String = output::generate_logfile_name();
        let logfile_path: path::PathBuf = log_directory.join(&log_name);

        match Logger::with_file(logfile_path) {
            Some(logger) => logger,
            None => {
                eprintln!("Error: Failed to create log file");
                return;
            }
        }
    } else {
        Logger::new()
    };

    if let Err(error) = match cli.command {
        Commands::Init(args) => init(&mut logger, &args),
    } {
        error!(logger, "{:#}", error);
        std::process::exit(1);
    }
}
