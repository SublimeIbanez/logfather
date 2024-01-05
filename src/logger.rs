use chrono::prelude::*;
use lazy_static::lazy_static;
use std::io::Write;
use ansi_term::Color;
use fs4::FileExt;

// TODO:
// 1. Implement advanced error handling for file operations
//    - Consider fallback strategies or silent error handling
// 2. Optimize performance for high-throughput logging
//    - Explore asynchronous logging
//    - Investigate file buffering
// 3. Add configurable timestamp formats
// 4. Conduct comprehensive testing
//    - Focus on concurrency and file writing
//    - Test log filtering behavior under various configurations

lazy_static! {
    static ref LOGGER: std::sync::RwLock<Logger> = std::sync::RwLock::new(Logger::new());
}

pub fn set_logger(new_logger: Logger) {
    let mut logger = LOGGER.write().expect("Could not access the logger");
    *logger = new_logger;
}

/// `Logger` is a struct that encapsulates the configuration for the logging system.
///
/// # Fields
/// - `path`: Optional path to the log file. If set, log messages will be written to this file.
/// - `terminal_output`: Boolean flag to enable or disable logging to the terminal.
/// - `file_output`: Boolean flag to enable or disable logging to a file.
/// - `output_level`: Minimum level of log messages to output. Messages below this level will be ignored.
/// - `format`: The format string for log messages. Placeholders like `{timestamp}`, `{module_path}`, `{level}`, and `{message}` will be replaced with actual values.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// let mut logger = Logger::new();
/// logger.terminal(true); // Enable terminal output
/// logger.file(true); // Enable file output
/// logger.path("log.txt"); // Set the path for file logging
/// logger.level(Level::Info); // Set the minimum log level to Info
/// logger.log_format("[{timestamp} {level}] {message}"); // Set a custom format for log messages
/// ```
#[derive(Clone)]
pub struct Logger {
    pub path: Option<String>,
    pub terminal_output: bool,
    pub file_output: bool,
    pub output_level: Level,
    pub log_format: String,
    //TODO: add other fields
    //pub timestamp_format: String,
}

impl Logger {
    /// Constructs a new `Logger` instance with default settings.
    ///
    /// By default, the logger is configured to:
    /// - Have no file path (no file logging).
    /// - Enable terminal output.
    /// - Disable file output.
    /// - Set the log level to `Info`.
    /// - Use a default format string for log messages.
    ///
    /// # Returns
    /// Returns a `Logger` instance with default settings.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let logger = Logger::new();
    /// ```
    pub fn new() -> Self {
        Self {
            path: None,
            terminal_output: true,
            file_output: false,
            output_level: Level::Info,
            log_format: String::from("[{timestamp} {module_path}] {level}: {message}")
        }
    }

    /// Sets the file path for the logger.
    ///
    /// If a path is set, the logger will write log messages to the specified file provided `file_output` is active.
    ///
    /// # Arguments
    /// * `path` - A string slice that holds the path to the log file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut logger = Logger::new();
    /// logger.path("/var/log/my_app.log");
    /// ```
    pub fn path(&mut self, path: &str) {
        self.path = Some(path.to_string());
    }

    /// Enables or disables terminal output for the logger - enabled by default.
    ///
    /// # Arguments
    /// * `value` - A boolean value where `true` enables terminal output and `false` disables it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut logger = Logger::new();
    /// logger.terminal(false); // Disable terminal output
    /// ```
    pub fn terminal(&mut self, value: bool) {
        self.terminal_output = value;
    }

    /// Enables or disables file output for the logger - disabled by default.
    ///
    /// Note: File output is only active if a file path is set.
    ///
    /// # Arguments
    /// * `value` - A boolean value where `true` enables file output and `false` disables it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut logger = Logger::new();
    /// logger.file(true); // Enable file output
    /// logger.path("/var/log/my_app.log"); // Set the path for file logging
    /// ```
    pub fn file(&mut self, value: bool) {
        self.file_output = value;
    }

    /// Sets the minimum output level for the logger.
    ///
    /// Log messages below this level will be ignored.
    ///
    /// # Arguments
    /// * `level` - The minimum `Level` of log messages to be output.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut logger = Logger::new();
    /// logger.level(Level::Warning); // Set the minimum log level to Warning - Info levels will not be logged
    /// ```
    pub fn level(&mut self, level: Level) {
        self.output_level = level;
    }

    /// Sets the format string for log messages.
    ///
    /// The format string can contain placeholders like `{timestamp}`, `{module_path}`, `{level}`, and `{message}` which will be replaced with actual values during logging.
    ///
    /// # Arguments
    /// * `format` - A string slice representing the log message format.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut logger = Logger::new();
    /// logger.log_format("{timestamp} - {level}: {message}"); // Set a custom format for log messages
    /// ```
    pub fn log_format(&mut self, format: &str) {
        self.log_format = format.to_string();
    }
}

/// Represents the severity level of a log message.
///
/// # Variants
///
/// - `Info`: Used for informational messages.
/// - `Warning`: Used for warning messages.
/// - `Error`: Used for error messages.
/// - `Critical`: Used for critical error messages that might require immediate attention.
/// - `None`: Special level used to disable logging.
///
/// # Examples
///
/// ```no_run
/// // Using `Level` to set the minimum output level of the logger
/// let mut logger = Logger::new();
/// logger.level(Level::Error); // Only log errors and critical messages
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Info = 0,
    Warning = 1,
    Error = 2,
    Critical = 3,
    None = 4,
}

impl ToString for Level {
    fn to_string(&self) -> String {
        match self {
            Level::Info => String::from("INFO"),
            Level::Warning => String::from("WARNING"),
            Level::Error => String::from("ERROR"),
            Level::Critical => String::from("CRITICAL"),
            Level::None => String::from("NONE"),
        }
    }
}

/// Logs a message with the specified log level and module path.
///
/// The log message is formatted according to the logger's configuration and output to the designated targets (file and/or terminal).
///
/// # Arguments
/// * `level` - The severity level of the log message.
/// * `module_path` - The module path where the log message originates.
/// * `message` - The log message.
///
/// # Examples
///
/// ```no_run
/// // Example of manually logging an error message
/// log(Level::Error, module_path!(), "An error occurred");
/// ```
///
/// Note: In practice, prefer using the provided macros (`info!`, `warning!`, `error!`, `critical!`) for logging.
pub fn log(level: Level, module_path: &str, message: &str) {
    //Grab a clone of the logger to not hold up any other potential logging threads
    let logger = LOGGER.read().expect("Could not read logger").clone();

    //If the level is too low then return
    if level < logger.output_level {
        return;
    }

    //Get the time
    let now = Local::now();
    let time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    //Replace the relevant sections in the format
    let log_format = logger.log_format
        .replace("{timestamp}", &time)
        .replace("{module_path}", module_path)
        .replace("{message}", message);

    //Only write to the file if both of these are true
    if logger.path.is_some() && logger.file_output {
        //Can safely unwrap
        let path = logger.path.unwrap();

        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path.as_str())
            .expect("Failed to open file");

        //Output-specific level replacement
        let format = log_format.replace("{level}", &level.to_string());

        //Lock down the file while it's being written to in case multithreaded application
        file.lock_exclusive().expect("Could not lock file for logging");
        match writeln!(file, "{}", format) { _ => () }  //Silent error handling 
        file.unlock().expect("Could not unlock file after writing");
    }

    //Terminal output
    if logger.terminal_output {
        //Set color
        //TODO: make this configurable by the user
        let level_color = match level {
            Level::Info => Color::Green.normal(),
            Level::Warning => Color::Yellow.normal(),
            Level::Error => Color::Red.normal(),
            Level::Critical => Color::Red.bold(),
            Level::None => Color::White.normal(),
        };

        //Output-specific level replacement
        let format = log_format.replace("{level}", &level_color.paint(&level.to_string()));

        //Print to the terminal
        println!("{}", format);
    }
}

/// Logs an informational message.
///
/// # Example
///
/// ```no_run
/// info!("This is an info message");
/// ```
#[macro_export]
macro_rules! info {
    ($message:expr) => {
        log(Level::Info, module_path!(), $message);
    };
}

/// Logs a warning message.
///
/// # Example
///
/// ```no_run
/// warning!("This is a warning message");
/// ```
///
/// This macro simplifies the process of logging a message at the `Warning` level.
#[macro_export]
macro_rules! warning {
    ($message:expr) => {
        log(Level::Warning, module_path!(), $message);
    };
}

/// Logs an error message.
///
/// # Example
///
/// ```no_run
/// error!("This is an error message");
/// ```
///
/// Use this macro for logging errors, typically when an operation fails or an unexpected condition occurs.
#[macro_export]
macro_rules! error {
    ($message:expr) => {
        log(Level::Error, module_path!(), $message);
    };
}

/// Logs a critical message.
///
/// # Example
///
/// ```no_run
/// critical!("This is a critical message");
/// ```
///
/// This macro is intended for critical errors that require immediate attention. Logging at this level typically indicates a serious failure in a component of the application.
#[macro_export]
macro_rules! critical {
    ($message:expr) => {
        log(Level::Critical, module_path!(), $message);
    };
}

pub use info;
pub use warning;
pub use error;
pub use critical;
