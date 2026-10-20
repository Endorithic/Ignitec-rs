use std::env;
use std::ffi;
use std::path;
use std::process;

use anyhow::Context;
use clap::Args;

use crate::logger::Logger;
use crate::subcommands::build;
use crate::subcommands::build::BuildArgs;
use crate::warn;

#[derive(Args)]
pub struct RunArgs {
    /// In case of rebuild, capture build args
    #[command(flatten)]
    build: BuildArgs,
    /// Whether the project should be rebuilt before running
    #[arg(short, long, default_value_t = false)]
    rebuild: bool,
    /// The arguments to pass to the program
    #[arg(last = true)]
    program_args: Vec<String>,
}

/// Runs the program executable if present, with optional rebuilding
pub fn run(logger: &mut Logger, args: &RunArgs) -> anyhow::Result<()> {
    if args.rebuild {
        build(logger, &args.build)?;
    }

    let cwd: path::PathBuf =
        env::current_dir().context("Failed to get current working directory.")?;
    let target_dir: path::PathBuf = cwd.join("target").join("debug");
    let project_name: &ffi::OsStr = cwd.file_name().unwrap_or(ffi::OsStr::new("app"));
    let target_path: path::PathBuf = target_dir.join(project_name);

    if target_path.exists() {
        let status: process::ExitStatus = process::Command::new(target_path)
            .args(&args.program_args)
            .status()?;

        match status.code() {
            Some(code) => std::process::exit(code),
            None => warn!(logger, "Process terminated with a signal."),
        }
    }

    anyhow::bail!("Failed to get executable. Maybe try building, or running run with -r?");
}
