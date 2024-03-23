use slog::{Drain, Logger, o};
use slog_async::Async;
use slog_json::Json;
use slog_term::{FullFormat, TermDecorator};

/// Configures and returns a `Logger` instance that outputs logs to both the terminal
/// and `stdout` in JSON format.
///
/// This function creates two separate logging drains:
/// - A terminal drain that formats logs with `slog_term`'s `FullFormat` for human-readable output.
/// - A JSON drain that formats logs as JSON with `slog_json` for structured logging.
///
/// Both drains are wrapped in asynchronous drains using `slog_async` to improve logging performance
/// by offloading the work to a dedicated thread. The asynchronous drains are then duplicated,
/// allowing log messages to be sent to both drains simultaneously.
///
/// # Returns
///
/// A `Logger` instance configured with the described drains. This logger can be used throughout
/// the application to log messages, which will appear in both the terminal and `stdout` in the
/// configured formats.
///
/// # Example
///
/// ```
/// // Initialize the logger
/// let log = configure_logging();
///
/// // Use the logger
/// slog::info!(log, "Application started"; "version" => "1.0.0");
/// ```
///
/// This will produce an output in the terminal in a human-readable format and also output a JSON
/// formatted log to `stdout`.
pub fn configure_logging() -> Logger {
    // Configure terminal logging
    let decorator = TermDecorator::new().build();
    let console_drain = FullFormat::new(decorator).build().fuse();
    // Make the console logging asynchronous
    let console_drain = Async::new(console_drain).build().fuse();

    // Configure JSON logging
    let json_drain = Json::new(std::io::stdout())
        .add_default_keys()
        .build().fuse();
    // Make the JSON logging asynchronous
    let json_drain = Async::new(json_drain).build().fuse();

    // Duplicate logs to both console and JSON output, and return the logger
    Logger::root(slog::Duplicate::new(console_drain, json_drain).fuse(), o!())
}
