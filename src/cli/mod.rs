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
    let maybe_backtrace = if std::env::var("RUST_BACKTRACE").is_err() {
        format!(
            "\nUse {} to make it more verbose",
            "RUST_BACKTRACE=1".italic().yellow()
        )
    } else {
        "".to_string()
    };

    let mut message = format!(
        "\n{}: {e}\n{maybe_backtrace}\n",
        "Phink got an error...".red().bold(),
    );

    if e.backtrace().status() == BacktraceStatus::Captured {
        message = format!(
            "{}\n{}\n{}",
            message,
            "More informations ->".yellow(),
            e.backtrace()
        )
    }

    let mut source = e.source();
    while let Some(cause) = source {
        let arrow = "—> ".on_bright_red().bold();
        message = format!("{message}\n{arrow} {cause}\n");
        source = cause.source();
    }

    message
}
