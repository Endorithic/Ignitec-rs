use std::fs::File;
use std::io::Write;
use std::path::Path;

use owo_colors::OwoColorize;

#[macro_export]
macro_rules! info {
    ($logger:ident, $($arg:tt)*) => {
        $logger.__info(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($logger:ident, $($arg:tt)*) => {
        $logger.__warn(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($logger:ident, $($arg:tt)*) => {
        $logger.__error(&format!($($arg)*))
    };
}

/// Used for printing log messages.
/// `Info` and `Warn` both send their messages to stdout, while `Error` sends to stderr.
/// Can optionally be constructed using a filepath for outputting messaages to a log path.
pub struct Logger {
    logfile: Option<File>,
}

impl Logger {
    /// Create a logger that only outputs to stdout/stderr
    pub fn new() -> Self {
        Self { logfile: None }
    }

    /// Create a logger that outputs both to stdout/stderr, and the specified
    /// logfile path. Returns None if the specified path's parent directory
    /// does not exist.
    pub fn with_file(path: impl AsRef<Path>) -> Option<Self> {
        match File::create(path) {
            Err(_) => None,
            Ok(file) => Some(Self {
                logfile: Some(file),
            }),
        }
    }

    /// Internal method for the info! macro.
    /// Is not supposed to be used directly.
    #[doc(hidden)]
    pub fn __info(&mut self, msg: &str) {
        println!("{}: {}", "[ INFO]".bright_green().bold(), msg);
        if let Some(file) = &mut self.logfile {
            let _ = writeln!(file, "{}: {}", "[ INFO]", msg);
        }
    }

    /// Internal method for the warn! macro.
    /// Is not supposed to be used directly.
    #[doc(hidden)]
    pub fn __warn(&mut self, msg: &str) {
        println!("{}: {}", "[ WARN]".bright_yellow().bold(), msg);
        if let Some(file) = &mut self.logfile {
            let _ = writeln!(file, "{}: {}", "[ WARN]", msg);
        }
    }

    /// Internal method for the error! macro.
    /// Is not supposed to be used directly.
    #[doc(hidden)]
    pub fn __error(&mut self, msg: &str) {
        eprintln!("{}: {}", "[ERROR]".bright_red().bold(), msg);
        if let Some(file) = &mut self.logfile {
            let _ = writeln!(file, "{}: {}", "[ERROR]", msg);
        }
    }
}
