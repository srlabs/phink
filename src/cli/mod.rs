use colored::Colorize;
use std::backtrace::BacktraceStatus;

pub mod config;
pub mod env;
pub mod ui;
pub mod ziggy;

/// Prints `anyhow` errors correctly in the Phink style
///
/// # Argument
/// * `e`: The `anyhow::Error` to pretty-print
///
/// returns: `String`
///
/// # Example
/// `eprintln!("{}", format_error(e));`
pub fn format_error(e: anyhow::Error) -> String {
    let mut message = format!("\n{}: {}\n", "Phink got an error...".red().bold(), e);

    if e.backtrace().status() == BacktraceStatus::Captured {
        message = format!(
            "{}\n{}\n{}\n",
            message,
            "Backtrace ->".yellow(),
            e.backtrace()
        )
    }

    let mut source = e.source();
    while let Some(cause) = source {
        message = format!("{}\n\n{} {}", message, "Caused by".cyan().bold(), cause);
        source = cause.source();
    }

    message
}
