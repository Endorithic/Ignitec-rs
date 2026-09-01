use std::fs;
use std::path;
use std::process;

use anyhow::Context;
use chrono::Local;
use directories::ProjectDirs;

use super::project_data;

/// Returns the directory in which to place the logs, and creates it if it
/// does not exist.
pub fn log_directory() -> anyhow::Result<path::PathBuf> {
    let dirs: ProjectDirs = ProjectDirs::from(
        project_data::PROJECT_QUALIFIER,
        project_data::PROJECT_ORG,
        project_data::PROJECT_NAME,
    )
    .context("Failed to get the log directory")?;

    let log_dir: path::PathBuf = dirs.data_local_dir().join("logs");
    fs::create_dir_all(&log_dir).context("Failed to create the log directory")?;

    Ok(log_dir)
}

/// Generates a logfile name based on the users current time, locale, and the process ID
pub fn generate_logfile_name() -> String {
    format!(
        "ignitec-{}-{}.log",
        Local::now().format("%Y-%m-%d_%H-%M-%S"),
        process::id()
    )
}
