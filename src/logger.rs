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
// 3. Conduct comprehensive testing
//    - Focus on concurrency and file writing
//    - Test log filtering behavior under various configurations

lazy_static! {
    static ref LOGFATHER: std::sync::RwLock<Logfather> = std::sync::RwLock::new(Logfather::new());
}

/// Sets the current Logfather struct
/// - Useful for if the log configuration was saved and reloaded
pub fn set_logfather(new_logfather: Logfather) {
    let mut logfather = LOGFATHER.write().expect("Could not access the logfather");
    *logfather = new_logfather;
}

/// `Logfather` is a struct that encapsulates the configuration for the logging system.
///
/// # Fields
/// - `path`: Optional path to the log file. If set, log messages will be written to this file.
/// - `terminal_output`: Boolean flag to enable or disable logging to the terminal.
/// - `file_output`: Boolean flag to enable or disable logging to a file.
/// - `output_level`: Minimum level of log messages to output. Messages below this level will be ignored.
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
/// let mut logfather = Logfather::new();
/// logfather.terminal(true); // Enable terminal output
/// logfather.file(true); // Enable file output
/// logfather.path("log.txt"); // Set the path for file logging
/// logfather.level(Level::Info); // Set the minimum log level to Info
/// logfather.log_format("[{timestamp} {level}] {message}"); // Set a custom format for log messages
/// logfather.timestamp_format("%Y-%m-%d %H:%M:%S"); // Set a custom format for timestamps
/// ```
#[derive(Clone)]
pub struct Logfather {
    pub path: Option<String>,
    pub terminal_output: bool,
    pub file_output: bool,
    pub output_level: Level,
    pub log_format: String,
    pub timestamp_format: String,
    //TODO: add other fields
}

impl Logfather {
    /// Constructs a new `Logfather` instance with default settings.
    ///
    /// By default, the logfather is configured to:
    /// - Have no file path (no file logging).
    /// - Enable terminal output.
    /// - Disable file output.
    /// - Set the log level to `Info`.
    /// - Use a default format string for log messages and timestamps.
    ///
    /// # Returns
    /// Returns a `Logfather` instance with default settings.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let logfather = Logfather::new();
    /// ```
    pub fn new() -> Self {
        Self {
            path: None,
            terminal_output: true,
            file_output: false,
            output_level: Level::Info,
            log_format: String::from("[{timestamp} {module_path}] {level}: {message}"),
            timestamp_format: String::from("%Y-%m-%d %H:%M:%S"),
        }
    }

    /// Sets the file path for the logfather.
    ///
    /// If a path is set, the logfather will write log messages to the specified file provided `file_output` is active.
    ///
    /// # Arguments
    /// * `path` - A string slice that holds the path to the log file.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let mut logfather = Logfather::new();
    /// logfather.path("/var/log/my_app.log");
    /// ```
    pub fn path(&mut self, path: &str) {
        self.path = Some(path.to_string());
        set_logfather(self.clone());
    }

    /// Enables or disables terminal output for the logfather - enabled by default.
    ///
    /// # Arguments
    /// * `value` - A boolean value where `true` enables terminal output and `false` disables it.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let mut logfather = Logfather::new();
    /// logfather.terminal(false); // Disable terminal output
    /// ```
    pub fn terminal(&mut self, value: bool) {
        self.terminal_output = value;
        set_logfather(self.clone());
    }

    /// Enables or disables file output for the logfather - disabled by default.
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
    /// let mut logfather = Logfather::new();
    /// logfather.file(true); // Enable file output
    /// logfather.path("/var/log/my_app.log"); // Set the path for file logging
    /// ```
    pub fn file(&mut self, value: bool) {
        self.file_output = value;
        set_logfather(self.clone());
    }

    /// Sets the minimum output level for the logfather.
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
    /// let mut logfather = Logfather::new();
    /// logfather.level(Level::Warning); // Set the minimum log level to Warning - Info levels will not be logged
    /// ```
    pub fn level(&mut self, level: Level) {
        self.output_level = level;
        set_logfather(self.clone());
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
    /// let mut logfather = Logfather::new();
    /// logfather.log_format("{timestamp} - {level}: {message}"); // Set a custom format for log messages
    /// ```
    pub fn log_format(&mut self, format: &str) {
        self.log_format = format.to_string();
        set_logfather(self.clone());
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
    /// let mut logfather = Logfather::new();
    /// logfather.timestamp_format("%m-%d-%y @%H:%M:%S"); // Set a custom format for timestamp display
    /// ```
    pub fn timestamp_format(&mut self, format: &str) {
        self.timestamp_format = format.to_string();
        set_logfather(self.clone());
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
/// ``` no_run
/// use logfather::*;
///
/// // Using `Level` to set the minimum output level of the logfather
/// let mut logfather = Logfather::new();
/// logfather.level(Level::Error); // Only log errors and critical messages
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
/// The log message is formatted according to the logfather's configuration and output to the designated targets (file and/or terminal).
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
    //Grab a clone of the logfather to not hold up any other potential logging threads
    let logfather = LOGFATHER.read().expect("Could not read logfather").clone();

    //If the level is too low then return
    if level < logfather.output_level {
        return;
    }

    //Get the time
    let now = Local::now();
    let time = now.format(&logfather.timestamp_format).to_string();

    //Replace the relevant sections in the format
    let log_format = logfather.log_format
        .replace("{timestamp}", &time)
        .replace("{module_path}", module_path)
        .replace("{message}", message);

    //Only write to the file if both of these are true
    if logfather.path.is_some() && logfather.file_output {
        //Can safely unwrap
        let path = logfather.path.unwrap();

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
    if logfather.terminal_output {
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
/// ``` no_run
/// use logfather::*;
///
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
/// ``` no_run
/// use logfather::*;
///
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
/// ``` no_run
/// use logfather::*;
///
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
/// ``` no_run
/// use logfather::*;
///
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{Read, Write};

    #[test]
    fn test_level_filtering() {
        let mut logfather = Logfather::new();
        logfather.level(Level::Error); //Error as basis

        //Test levels below
        assert!(Level::Info < logfather.output_level);
        assert!(Level::Warning < logfather.output_level);

        //Test levels above
        assert!(Level::Error >= logfather.output_level);
        assert!(Level::Critical >= logfather.output_level);
    }

    #[test]
    fn test_level_none() {
        let mut logfather = Logfather::new();
        logfather.level(Level::None); //Set to None

        //Test levels below
        assert!(Level::Info < logfather.output_level);
        assert!(Level::Warning < logfather.output_level);
        assert!(Level::Error < logfather.output_level);
        assert!(Level::Critical < logfather.output_level);
    }

    #[test]
    fn test_log_format() {
        let mut logfather = Logfather::new();
        logfather.log_format("{level} - {message}");

        let formatted_message = logfather.log_format
            .replace("{level}", "INFO")
            .replace("{message}", "Test message");

        assert_eq!(formatted_message, "INFO - Test message");
    }

    #[test]
    fn test_file_output() {
        let mut logfather = Logfather::new();
        let test_file_path = "test_log.txt";

        // Enable file output and set file path
        logfather.file(true);
        logfather.path(test_file_path);

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
        logfather.file(false);
        log(Level::Info, "test_file_output", "Second test message for file output");

        // Verify that no file is created or written to
        assert!(!fs::metadata(test_file_path).is_ok(), "Log file should not exist when file output is disabled");
    }

    #[test]
    fn test_log_level_filtering() {
        let mut logfather = Logfather::new();
        let test_file_path = "test_log_level.txt";

        let _ = fs::remove_file(test_file_path);

        // Set file output and path
        logfather.file(true);
        logfather.path(test_file_path);

        // Set log level to Warning and log an Info message (should not be logged)
        logfather.level(Level::Warning);
        log(Level::Info, "test_log_level_filtering", "Info level message");
        
        // Verify that the Info message is not in the file
        let mut file = File::open(test_file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert!(!contents.contains("Info level message"), "Info level message should not be logged");

        // Log a Warning message (should be logged)
        log(Level::Warning, "test_log_level_filtering", "Warning level message");

        // Read file again and check for the Warning message
        file = File::open(test_file_path).unwrap();
        contents.clear();
        file.read_to_string(&mut contents).unwrap();
        assert!(contents.contains("Warning level message"), "Warning level message should be logged");

        // Clean up: remove the test log file
        fs::remove_file(test_file_path).expect("Failed to delete test log file");
    }
}
