use chrono::{prelude::Local, Utc};
use lazy_static::lazy_static;
use std::io::Write;
use fs4::FileExt;

// TODO:
// 1. Implement advanced error handling for file operations
//    - Consider fallback strategies or silent error handling
// 2. Optimize performance for high-throughput logging
//    - Explore asynchronous logging
//    - Investigate file buffering
// 3. Conduct comprehensive testing
//    - Focus on concurrency and file writing
//    - Test log filtering behavior under various configurations

lazy_static! {
    static ref LOGGER: std::sync::RwLock<Logger> = std::sync::RwLock::new(Logger::new());
}


/// Sets the current Logger struct
/// - Useful for if the log configuration was saved and reloaded
pub fn set_logger(new_logger: &Logger) {
    let mut logger = LOGGER.write().expect("Could not access the logger");
    *logger = new_logger.clone();
}

/// `Logger` is a struct that encapsulates the configuration for the logging system.
///
/// # Fields
/// - `path`: Optional path to the log file. If set, log messages will be written to this file.
/// - `terminal_output`: Boolean flag to enable or disable logging to the terminal.
/// - `file_output`: Boolean flag to enable or disable logging to a file.
/// - `output_level`: Minimum level of log messages to output. Messages below this level will be ignored.
/// - `ignore`: Global list of log levels to ignore - more granular than output_level.
/// - `file_ignore`: List of log levels that file output ignores.
/// - `terminal_ignore`: List of log levels that terminal output ignores.
/// - `log_format`: The format string for log messages. Placeholders like `{timestamp}`, `{module_path}`, `{level}`, and `{message}` will be replaced with actual values.
/// - `timestampt_format`: The format string for time display. Placeholders like `%y`, `%m`, `%d`, `%H`, `%M`, and `%S` will be replaced with actual values.
///
/// # Examples
///
/// Basic usage:
///
/// ``` no_run
/// use logfather::*;
///
/// let mut logger = Logger::new();
/// logger.terminal(true); // Enable terminal output
/// logger.file(true); // Enable file output
/// logger.path("log.txt"); // Set the path for file logging
/// logger.level(Level::Info); // Set the minimum log level to Info
/// logger.ignore(Level::Error); // Globally ignore the Error level messages
/// logger.file_ignore(Level::Error); //Ignores the Error level messages for file output
/// logger.terminal_ignore(Level::Error); //Ignores the Error level messages for terminal output
/// logger.log_format("[{timestamp} {level}] {message}"); // Set a custom format for log messages
/// logger.timestamp_format("%Y-%m-%d %H:%M:%S"); // Set a custom format for timestamps
/// ```
#[derive(Clone)]
pub struct Logger {
    pub(crate) path: Option<String>,
    pub(crate) terminal_output: bool,
    pub(crate) file_output: bool,
    pub(crate) output_level: Level,
    pub(crate) ignore: Vec<Level>,
    pub(crate) file_ignore: Vec<Level>,
    pub(crate) terminal_ignore: Vec<Level>,
    pub(crate) log_format: String,
    pub(crate) timezone: TimeZone,
    pub(crate) timestamp_format: String,
    //TODO: add other fields
}

impl Logger {
    /// Constructs a new `Logger` instance with default settings.
    ///
    /// By default, the logger is configured to:
    /// - Have no file path (no file logging).
    /// - Enable terminal output.
    /// - Disable file output.
    /// - Set the log level to `Info`.
    /// - Use a default format string for log messages and timestamps.
    ///
    /// # Returns
    /// Returns a `Logger` instance with default settings.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let logger = Logger::new();
    /// ```
    pub fn new() -> Self {
        Self {
            path: None,
            terminal_output: true,
            file_output: false,
            output_level: Level::Trace,
            ignore: Vec::new(),
            file_ignore: Vec::new(),
            terminal_ignore: Vec::new(),
            log_format: String::from("[{timestamp} {level} {module_path}] {message}"),
            timezone: TimeZone::Local,
            timestamp_format: String::from("%Y-%m-%d %H:%M:%S"),
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
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let mut logger = Logger::new();
    /// logger.path("/var/log/my_app.log");
    /// ```
    pub fn path(&mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        set_logger(self);
        return self.to_owned();
    }

    /// Enables or disables terminal output for the logger - enabled by default.
    ///
    /// # Arguments
    /// * `value` - A boolean value where `true` enables terminal output and `false` disables it.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let mut logger = Logger::new();
    /// logger.terminal(false); // Disable terminal output
    /// ```
    pub fn terminal(&mut self, value: bool) -> Self {
        self.terminal_output = value;
        set_logger(self);
        return self.to_owned();
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
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let mut logger = Logger::new();
    /// logger.file(true); // Enable file output
    /// logger.path("/var/log/my_app.log"); // Set the path for file logging
    /// ```
    pub fn file(&mut self, value: bool) -> Self {
        self.file_output = value;
        set_logger(self);
        return self.to_owned();
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
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let mut logger = Logger::new();
    /// logger.level(Level::Warning); // Set the minimum log level to Warning - Info levels will not be logged
    /// ```
    pub fn level(&mut self, level: Level) -> Self {
        self.output_level = level;
        set_logger(self);
        return self.to_owned();
    }

    /// Adds a level to ignore to the list.
    ///
    /// Log messages of this level will be ignored.
    ///
    /// # Arguments
    /// * `level` - The `Level` of log messages to be ignored.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let mut logger = Logger::new();
    /// logger.ignore(Level::Warning); // Messages of `Warning` level will be ignored
    /// ```
    pub fn ignore(&mut self, level: Level) -> Self {
        self.ignore.push(level);
        set_logger(self);
        return self.to_owned();
    }

    /// Adds a level to ignore to the file_ignore list.
    ///
    /// Log messages of this level will be ignored when writing to the file.
    ///
    /// # Arguments
    /// * `level` - The `Level` of log messages to be ignored.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let mut logger = Logger::new();
    /// logger.file_ignore(Level::Warning); // Messages of `Warning` level will be ignored
    /// ```
    pub fn file_ignore(&mut self, level: Level) -> Self {
        self.file_ignore.push(level);
        set_logger(self);
        return self.to_owned();
    }

    /// Adds a level to ignore to the terminal_ignore list.
    ///
    /// Log messages of this level will be ignored when writing to the terminal.
    ///
    /// # Arguments
    /// * `level` - The `Level` of log messages to be ignored.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let mut logger = Logger::new();
    /// logger.terminal_ignore(Level::Warning); // Messages of `Warning` level will be ignored
    /// ```
    pub fn terminal_ignore(&mut self, level: Level) -> Self {
        self.terminal_ignore.push(level);
        set_logger(self);
        return self.to_owned();
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
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let mut logger = Logger::new();
    /// logger.log_format("{timestamp} - {level}: {message}"); // Set a custom format for log messages
    /// ```
    pub fn log_format(&mut self, format: &str) -> Self {
        self.log_format = format.to_string();
        set_logger(&self);
        return self.to_owned();
    }

    /// Sets the preferred timezone for the log display.
    ///
    /// # Arguments
    /// * `timezone` - A value which depicts Local or UTC timezone preference.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let mut logger = Logger::new();
    /// logger.timezone(TimeZone::Utc); // Set timezone to either be local or utc
    /// ```
    pub fn timezone(&mut self, timezone: TimeZone) -> Self {
        self.timezone = timezone;
        set_logger(&self);
        return self.to_owned();
    }

    /// Sets the format string for timestamps within the log.
    ///
    /// The format string can contain placeholders like:
    /// - `%y`, `%m`, `%d` for year, month, and day, respectively.
    /// - `%H`, `%M`, `%S` for hour, minute, and second, respectively.
    ///
    /// # Arguments
    /// * `format` - A string slice representing the timestamp format.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let mut logger = Logger::new();
    /// logger.timestamp_format("%m-%d-%y @%H:%M:%S"); // Set a custom format for timestamp display
    /// ```
    pub fn timestamp_format(&mut self, format: &str) -> Self {
        self.timestamp_format = format.to_string();
        set_logger(self);
        return self.to_owned();
    }
}

/// Represents the severity level of a log message.
///
/// # Variants
///
/// - `Trace`: Used for standard output - lowest level.
/// - `Debug`: Used for debug messages.
/// - `Info`: Used for informational messages.
/// - `Warning`: Used for warning messages.
/// - `Error`: Used for error messages.
/// - `Critical`: Used for critical error messages that might require immediate attention.
/// - `None`: Special level used to disable logging.
///
/// # Values:
/// None > Critical > Error > Warning > Info > Debug > Trace
///
/// # Examples
///
/// ``` no_run
/// use logfather::*;
///
/// // Using `Level` to set the minimum output level of the logger
/// let mut logger = Logger::new();
/// logger.level(Level::Error); // Only log errors and critical messages
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
    Critical = 5,
    None = 255,
}

impl ToString for Level {
    fn to_string(&self) -> String {
        let value = match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warning => "WARNING",
            Level::Error => "ERROR",
            Level::Critical => "CRITICAL",
            Level::None => "NONE",
        };
        return String::from(value);
    }
}

/// TimeZone selection.
///
/// # Variants
///
/// - `Local`: Time will be displayed in local time (default).
/// - `Utc`: Time will be displayed in Zulu (UTC) time.
///
/// # Examples
///
/// ``` no_run
/// use logfather::*;
///
/// let mut logger = Logger::new();
/// logger.timezone(TimeZone::Utc); // Sets timezone to UTC
/// ```
#[derive(Clone)]
pub enum TimeZone {
    Local,
    Utc,
}

pub enum Color {
    BlackText,
    RedText,
    GreenText,
    YellowText,
    BlueText,
    MagentaText,
    CyanText,
    WhiteText,
    BlackHighlight,
    RedHighlight,
    GreenHighlight,
    YellowHighlight,
    BlueHighlight,
    MagentaHighlight,
    CyanHighlight,
    WhiteHighlight,
    Reset,
}

impl Color {
    pub fn to_string(&self) -> String {
        let value = match self {
            Color::BlackText => "\x1b[30m",
            Color::RedText => "\x1b[31m",
            Color::GreenText => "\x1b[32m",
            Color::YellowText => "\x1b[33m",
            Color::BlueText => "\x1b[34m",
            Color::MagentaText => "\x1b[35m",
            Color::CyanText => "\x1b[36m",
            Color::WhiteText => "\x1b[37m",
            Color::BlackHighlight => "\x1b[40m",
            Color::RedHighlight => "\x1b[41m",
            Color::GreenHighlight => "\x1b[42m",
            Color::YellowHighlight => "\x1b[43m",
            Color::BlueHighlight => "\x1b[44m",
            Color::MagentaHighlight => "\x1b[45m",
            Color::CyanHighlight => "\x1b[46m",
            Color::WhiteHighlight => "\x1b[47m",
            Color::Reset => "\x1b[0m",
        };
        return String::from(value);
    }

    pub fn bold(&self) -> String {
        return format!("{}{}", "\x1b[1b", self.to_string());
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
/// ``` no_run
/// use logfather::*;
///
/// // Example of manually logging an error message
/// log(Level::Error, module_path!(), "An error occurred");
/// ```
///
/// Note: In practice, prefer using the provided macros (`info!`, `warning!`, `error!`, `critical!`) for logging.
pub fn log(level: Level, module_path: &str, message: &str) {
    //Grab a clone of the logger to not hold up any other potential logging threads
    let logger = LOGGER.read().expect("Could not read logger").clone();

    //If the level is too low then return
    if level < logger.output_level || logger.ignore.contains(&level) {
        return;
    }

    //Get the time
    let time = match logger.timezone {
        TimeZone::Local => {
            let now = Local::now();
            now.format(&logger.timestamp_format).to_string()
        },
        TimeZone::Utc => {
            let now = Utc::now();
            now.format(&logger.timestamp_format).to_string()
        },
    };

    //Replace the relevant sections in the format
    let log_format = logger.log_format
        .replace("{timestamp}", &time)
        .replace("{module_path}", module_path)
        .replace("{message}", message);

    //Only write to the file if both of these are true
    if logger.path.is_some() && logger.file_output && !logger.file_ignore.contains(&level) {
        //Can safely unwrap
        let path = logger.path.unwrap();

        let mut file = std::fs::OpenOptions::new()
            .read(true)
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
    if logger.terminal_output && !logger.terminal_ignore.contains(&level) {
        //Set color
        //TODO: make this configurable by the user
        let level_color = match level {
            Level::Trace => Color::WhiteText.to_string(), // No color
            Level::Debug=> Color::BlueText.to_string(), // Blue
            Level::Info => Color::GreenText.to_string(), // Green
            Level::Warning => Color::YellowText.to_string(), // Yellow
            Level::Error => Color::RedText.to_string(), // Red
            Level::Critical => Color::RedText.bold(), //Bold Red
            Level::None => Color::Reset.to_string(), // Retain for addition purposes - code that resets the text
        };

        //Output-specific level replacement
        let format = log_format.replace(
            "{level}", &format!("{}{}{}", level_color, level.to_string(), Color::Reset.to_string())
        );

        //Print to the terminal
        println!("{}", format);

    }
}

/// Logs a message for tracing - very low priority.
///
/// # Example
///
/// ``` no_run
/// use logfather::trace;
///
/// trace!("This is a debug message");
/// ```
#[macro_export]
macro_rules! trace {
    ($message:expr) => {
        $crate::log($crate::Level::Trace, module_path!(), $message);
    };
}

/// Logs a message for debugging.
///
/// # Example
///
/// ``` no_run
/// use logfather::debug;
///
/// debug!("This is a debug message");
/// ```
#[macro_export]
macro_rules! debug {
    ($message:expr) => {
        $crate::log($crate::Level::Debug, module_path!(), $message);
    };
}

/// Logs an informational message.
///
/// # Example
///
/// ``` no_run
/// use logfather::info;
///
/// info!("This is an info message");
/// ```
#[macro_export]
macro_rules! info {
    ($message:expr) => {
        $crate::log($crate::Level::Info, module_path!(), $message);
    };
}

/// Logs a warning message.
///
/// # Example
///
/// ``` no_run
/// use logfather::warning;
///
/// warning!("This is a warning message");
/// ```
///
/// This macro simplifies the process of logging a message at the `Warning` level.
#[macro_export]
macro_rules! warning {
    ($message:expr) => {
        $crate::log($crate::Level::Warning, module_path!(), $message);
    };
}

/// Logs a warning message.
///
/// # Example
///
/// ``` no_run
/// use logfather::warn;
///
/// warn!("This is a warning message");
/// ```
///
/// This macro simplifies the process of logging a message at the `Warning` level.
#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        $crate::log($crate::Level::Warning, module_path!(), $message);
    };
}

/// Logs an error message.
///
/// # Example
///
/// ``` no_run
/// use logfather::error;
///
/// error!("This is an error message");
/// ```
///
/// Use this macro for logging errors, typically when an operation fails or an unexpected condition occurs.
#[macro_export]
macro_rules! error {
    ($message:expr) => {
        $crate::log($crate::Level::Error, module_path!(), $message);
    };
}

/// Logs a critical message.
///
/// # Example
///
/// ``` no_run
/// use logfather::critical;
///
/// critical!("This is a critical message");
/// ```
///
/// This macro is intended for critical errors that require immediate attention. Logging at this level typically indicates a serious failure in a component of the application.
#[macro_export]
macro_rules! critical {
    ($message:expr) => {
        $crate::log($crate::Level::Critical, module_path!(), $message);
    };
}

/// Logs a critical message.
///
/// # Example
///
/// ``` no_run
/// use logfather::crit;
///
/// crit!("This is a critical message");
/// ```
///
/// This macro is intended for critical errors that require immediate attention. Logging at this level typically indicates a serious failure in a component of the application.
#[macro_export]
macro_rules! crit {
    ($message:expr) => {
        $crate::log($crate::Level::Critical, module_path!(), $message);
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Read;

    #[test]
    fn test_level_filtering() {
        let mut logger = Logger::new();
        logger.level(Level::Error); //Error as basis

        //Test levels below
        assert!(Level::Info < logger.output_level);
        assert!(Level::Debug < logger.output_level);
        assert!(Level::Warning < logger.output_level);

        //Test levels equal-to-or-above
        assert!(Level::Error >= logger.output_level);
        assert!(Level::Critical >= logger.output_level);
    }

    #[test]
    fn test_level_none() {
        let mut logger = Logger::new();
        logger.level(Level::None); //Set to None

        //Test levels below
        assert!(Level::Info < logger.output_level);
        assert!(Level::Debug < logger.output_level);
        assert!(Level::Warning < logger.output_level);
        assert!(Level::Error < logger.output_level);
        assert!(Level::Critical < logger.output_level);
    }

    #[test]
    fn test_log_format() {
        let mut logger = Logger::new();
        logger.log_format("{level} - {message}");

        let formatted_message = logger.log_format
            .replace("{level}", "INFO")
            .replace("{message}", "Test message");

        assert_eq!(formatted_message, "INFO - Test message");
    }

    #[test]
    fn test_file_output() {
        let mut logger = Logger::new();
        let test_file_path = "test_log.txt";

        // Enable file output and set file path
        logger.file(true);
        logger.path(test_file_path);

        // Log a message
        log(Level::Info, "test_file_output", "Test log message for file output");
        
        // Verify that file contains the logged message
        let mut file = File::open(test_file_path).expect("Failed to open log file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read log file");
        assert!(contents.contains("Test log message for file output"), "Log message not found in file");

        // Clean up: remove the test log file
        fs::remove_file(test_file_path).expect("Failed to delete test log file");

        // Disable file output and attempt to log another message
        logger.file(false);
        log(Level::Info, "test_file_output", "Second test message for file output");

        // Verify that no file is created or written to
        assert!(!fs::metadata(test_file_path).is_ok(), "Log file should not exist when file output is disabled");
    }
}
