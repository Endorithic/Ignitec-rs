use std::env;
use std::fs;
use std::path;

use anyhow::Context;
use clap::Args;

use crate::info;
use crate::logger::Logger;

#[derive(Args)]
pub struct InitArgs {
    /// The name of the project to initialize
    name: String,
    /// Whether the project should generate a .clangd file
    #[arg(long, default_value_t = false)]
    clangd: bool,
}

/// Cleans up the created project structure if the init process fails
struct InitGuard {
    root: path::PathBuf,
    is_valid: bool,
}

impl InitGuard {
    pub fn new(root: path::PathBuf) -> Self {
        Self {
            root: root,
            is_valid: false,
        }
    }

    pub fn commit(&mut self) {
        self.is_valid = true;
    }
}

impl Drop for InitGuard {
    fn drop(&mut self) {
        if !self.is_valid {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

pub fn init(logger: &mut Logger, args: &InitArgs) -> anyhow::Result<()> {
    let cwd: path::PathBuf = env::current_dir().context("Failed to get the current directory")?;
    let project_dir: path::PathBuf = cwd.join(&args.name);
    if project_dir.exists() {
        anyhow::bail!("Directory `{}` already exists", args.name);
    }

    info!(logger, "Creating project directory `{}`", args.name);
    fs::create_dir(&project_dir).context("Failed to create project directory")?;

    let mut guard: InitGuard = InitGuard::new(project_dir.clone());

    let src_dir: path::PathBuf = project_dir.join("src");
    let include_dir: path::PathBuf = project_dir.join("include");
    let project_include_dir: path::PathBuf = include_dir.join(&args.name);

    info!(logger, "Creating source directories");
    fs::create_dir(&src_dir).context("Failed to create source directory")?;
    fs::create_dir(&include_dir).context("Failed to create include directory")?;
    fs::create_dir(&project_include_dir).context("Failed to create project include directory")?;

    let target_dir: path::PathBuf = project_dir.join("target");
    let debug_dir: path::PathBuf = target_dir.join("debug");

    info!(logger, "Creating target directories");
    fs::create_dir(&target_dir).context("Failed to create target directory")?;
    fs::create_dir(&debug_dir).context("Failed to create debug directory")?;

    info!(logger, "Writing `src/main.cpp`");
    fs::write(
        src_dir.join("main.cpp"),
        "#include <print>\n\nint main() {\n    std::println(\"Hello world!\");\n}\n",
    )
    .context("Failed to write `src/main.cpp`")?;

    info!(logger, "Writing `.clangd`");
    fs::write(
        project_dir.join(".clangd"),
        "CompileFlags:\n    Add: [-std=c++23, -I../include/]\n",
    )
    .context("Failed to write `.clangd`")?;

    guard.commit();

    Ok(())
}
