use crate::output::{file_output, file_roller, terminal_output};
use dekor::*;
use lazy_static::lazy_static;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};
use std::{collections::HashMap, path::PathBuf, sync::RwLock};

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
    pub static ref LOGGER: RwLock<Logger> = RwLock::new(Logger::default());
    pub static ref TERMINAL_BUFFER: RwLock<Vec<String>> = RwLock::new(Vec::new());
    pub static ref FILE_BUFFER: RwLock<Vec<String>> = RwLock::new(Vec::new());
    pub static ref FILE: RwLock<Option<std::fs::File>> = RwLock::new(None);
    pub static ref LOCKED: RwLock<bool> = RwLock::new(false);
}

/// Replaces the current global logger instance with a new one.
///
/// This function allows updating the global logger configuration at runtime. It should be used
/// with caution to avoid race conditions in multi-threaded environments. The new logger configuration
/// is immediately applied to all subsequent log operations.
///
/// # Arguments
/// * `new_logger` - A reference to the `Logger` instance that will replace the current global logger.
///
/// # Panics
/// Panics if the global logger lock cannot be acquired.
///
/// # Examples
/// ```
/// use logfather::*;
///
/// let mut custom_logger = Logger::new();
/// custom_logger.level(Level::Warning); // Set custom logging level
/// logfather::logger::set_logger(&custom_logger); // Apply the custom logger globally
/// ```
#[inline]
pub fn set_logger(new_logger: &Logger) {
    *LOGGER.write().expect("Could not set new logger") = new_logger.clone();
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
/// - `styles`: HashMap relating a `Level` to a `TextStyle` for terminal output customization.
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
/// logger.structured_format("\n\t{key}: {value}"); // Set custom format for structured log messages
/// logger.timestamp_format("%Y-%m-%d %H:%M:%S"); // Set a custom format for timestamps
/// logger.add_style(Level::Info, Style::Underline); // Set the style for INFO to Underlined in terminal output
/// ```
#[derive(Clone, Debug)]
pub struct Logger {
    // Terminal
    pub terminal_output: bool,
    pub terminal_std_output: OutputDirection,
    pub terminal_ignore: Vec<Level>,
    pub terminal_buffer_interval: std::time::Duration,
    // File
    pub file_output: bool,
    pub file_path: Option<PathBuf>,
    pub file_ignore: Vec<Level>,
    pub file_buffer_interval: std::time::Duration,
    pub file_rollover: usize,
    // General
    pub output_level: Level,
    pub ignore: Vec<Level>,
    pub log_format: String,
    pub structured_format: String,
    pub timezone: TimeZone,
    pub timestamp_format: String,
    pub styles: HashMap<Level, Vec<Style>>,
}

#[derive(Clone, Debug)]
pub enum OutputDirection {
    Stderr,
    Stdout,
}

impl Default for Logger {
    fn default() -> Self {
        let logger = Self {
            // Terminal
            terminal_output: true,
            terminal_ignore: vec![],
            terminal_buffer_interval: std::time::Duration::from_nanos(0),
            terminal_std_output: OutputDirection::Stderr,
            // File
            file_output: false,
            file_path: None,
            file_ignore: vec![],
            file_buffer_interval: std::time::Duration::from_nanos(0),
            file_rollover: 0,
            // General
            output_level: Level::Trace,
            ignore: vec![],
            log_format: String::from("[{timestamp} {level} {module_path}] {message}"),
            structured_format: String::from(" {key}: {value}"),
            timezone: TimeZone::Local,
            timestamp_format: String::from("%Y-%m-%d %H:%M:%S"),
            styles: HashMap::from([
                (Level::Trace, vec![Style::FGPurple]),
                (Level::Debug, vec![Style::FGBlue]),
                (Level::Info, vec![Style::FGGreen]),
                (Level::Warning, vec![Style::FGYellow]),
                (Level::Error, vec![Style::FGRed]),
                (Level::Critical, vec![Style::Bold, Style::FGRed]),
                (Level::Diagnostic, vec![Style::Bold, Style::FGCyan]),
                (Level::None, vec![]),
            ]),
        };

        let _terminal_output = std::thread::spawn(|| {
            terminal_output();
        });

        let _file_output = std::thread::spawn(|| {
            file_output();
        });

        let _file_roller = std::thread::spawn(|| {
            file_roller();
        });

        return logger;
    }
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
        return Self::default();
    }

    pub fn current_clone() -> Self {
        return LOGGER.read().expect("Could not get current logger").clone();
    }

    pub fn read_current<'a>() -> RwLockReadGuard<'a, Self> {
        return LOGGER.read().expect("Could not get current logger");
    }

    pub fn write_current<'a>() -> RwLockWriteGuard<'a, Self> {
        return LOGGER.write().expect("Could not get current logger");
    }

    // ##################################################################### Terminal #####################################################################
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
    #[inline]
    pub fn terminal(&self, value: bool) -> Self {
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .terminal_output = value;
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: terminal")
            .clone();
    }

    #[inline]
    pub fn terminal_std_output(&mut self, value: OutputDirection) -> Self {
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .terminal_std_output = value;
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: std_output")
            .clone();
    }

    #[inline]
    pub fn terminal_buffer_interval(&mut self, duration: std::time::Duration) -> Self {
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .terminal_buffer_interval = duration;
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: terminal_buffer_interval")
            .clone();
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
    #[inline]
    pub fn terminal_ignore(&mut self, level: Level) -> Self {
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .terminal_ignore
                .push(level);
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: terminal_ignore")
            .clone();
    }

    // ####################################################################### File #######################################################################
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
    #[inline]
    pub fn file(&mut self, value: bool) -> Self {
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .file_output = value;
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: file")
            .clone();
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
    #[inline]
    pub fn path(&mut self, path: &str) -> Self {
        // Create parent directories if necessary.
        if let Some(parent) = PathBuf::from(&path).parent() {
            std::fs::create_dir_all(parent).expect("Could not create parent directories.");
        }
        // Create the path
        let mut p = PathBuf::from(path);
        // Handle empty path
        if p.as_os_str().is_empty() {
            // Get the current directory
            p = std::env::current_dir().expect("Could not open path");
            // Append default file name
            p.push(".logger");
        }
        {
            *FILE.write().expect("Could not overwrite FILE") = Some(
                std::fs::OpenOptions::new()
                    .create(true)
                    .read(true)
                    .write(true)
                    .truncate(false)
                    .open(&p)
                    .expect("Could not open file"),
            );
        }
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .file_path = Some(p);
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: path")
            .clone();
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
    #[inline]
    pub fn file_ignore(&mut self, level: Level) -> Self {
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .file_ignore
                .push(level);
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: level")
            .clone();
    }

    #[inline]
    pub fn file_buffer_interval(&mut self, duration: std::time::Duration) -> Self {
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .file_buffer_interval = duration;
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: file_interval")
            .clone();
    }

    #[inline]
    pub fn file_rollover(&mut self, lines: usize) -> Self {
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .file_rollover = lines;
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: file_interval")
            .clone();
    }

    // ##################################################################### General ######################################################################
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
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .output_level = level;
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: level")
            .clone();
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
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .ignore
                .push(level);
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: ignore")
            .clone();
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
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .log_format = String::from(format);
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: log_format")
            .clone();
    }

    /// Sets the format string for log messages in structured logging.
    ///
    /// The format string can contain placeholders like `{key}` and `{value}` which will be replaced during logging.
    ///
    /// # Arguments
    /// * `format` - A string slice representing the log message format for structured logs.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let mut logger = Logger::new();
    /// logger.structured_format("\n\t{key}: {value}"); // Set a custom format for log messages
    /// ```
    pub fn structured_format(&mut self, format: &str) -> Self {
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .log_format = String::from(format);
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: log_format")
            .clone();
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
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .timezone = timezone;
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: timezone")
            .clone();
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
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .timestamp_format = String::from(format);
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: timestamp_format")
            .clone();
    }

    /// Sets the text styles for a specific log level.
    ///
    /// This function allows customizing the appearance of log messages in the terminal based on their
    /// level. Each level can be associated with a set of styles, such as color or text decoration (e.g., bold, underline).
    ///
    /// # Arguments
    /// * `level` - The log level whose output style will be modified.
    /// * `style_set` - A vector of `Style` enums specifying the styles to apply to the specified log level.
    ///
    /// # Returns
    /// Returns the modified `Logger` instance to allow for method chaining.
    ///
    /// # Examples
    /// ```
    /// use logfather::*;
    ///
    /// let mut logger = Logger::new();
    /// logger.style(Level::Info, vec![Style::FGGreen, Style::Bold]); // Set INFO messages to be green and bold
    /// ```
    pub fn style(&mut self, level: Level, style_set: Vec<Style>) -> Self {
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .styles
                .insert(level, style_set);
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: style")
            .clone();
    }

    /// Adds a style to the list of styles for a specified log level.
    ///
    /// This function is useful for incrementally building up the styling of log messages at different
    /// levels without replacing the existing styles. It adds a single `TextStyle` to the list of styles
    /// associated with a given log level.
    ///
    /// # Arguments
    /// * `level` - The log level to modify.
    /// * `style` - The style to add to the list of styles for the specified level.
    ///
    /// # Panics
    /// Panics if the log level does not already have an associated list of styles.
    ///
    /// # Returns
    /// Returns the modified `Logger` instance to allow for method chaining.
    ///
    /// # Examples
    /// ```
    /// use logfather::*;
    ///
    /// let mut logger = Logger::new();
    /// logger.add_style(Level::Error, Style::FGRed); // Add red color to ERROR level messages
    /// ```
    pub fn add_style(&mut self, level: Level, style: Style) -> Self {
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .styles
                .get_mut(&level)
                .expect("Could not get style level")
                .push(style);
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: add_style")
            .clone();
    }

    /// Removes a style from the list of styles for a given level.
    ///
    /// # Arguments
    /// * `level` - The Level being modified
    /// * `style` - The Style being removed from the `TextStyle` enum
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use logfather::*;
    ///
    /// let mut logger = Logger::new();
    /// logger.remove_style(Level::Info, Style::BGBlue);
    /// ```
    pub fn remove_style(&mut self, level: Level, style: Style) -> Self {
        {
            LOGGER
                .write()
                .expect("Could not get logger for overwrite function.")
                .styles
                .get_mut(&level)
                .expect("Could not get style level")
                .retain(|s| *s != style);
        }
        return LOGGER
            .read()
            .expect("Could not return logger clone: remove_style")
            .clone();
    }

    /// Retrieves a copy of the styles associated with a specific log level.
    ///
    /// This function returns the current list of styles applied to log messages of the specified level.
    /// It is useful for inspecting or dynamically adjusting the logging output based on the application's state.
    ///
    /// # Arguments
    /// * `level` - The log level whose styles will be retrieved.
    ///
    /// # Returns
    /// Returns a vector of `Style` enums representing the styles associated with the specified log level.
    ///
    /// # Panics
    /// Panics if there are no styles associated with the specified log level.
    ///
    /// # Examples
    /// ```
    /// use logfather::*;
    ///
    /// let logger = Logger::new();
    /// let info_styles = logger.styles(Level::Info); // Retrieve styles for INFO level messages
    /// ```
    pub fn styles(&self, level: Level) -> Vec<Style> {
        return self
            .styles
            .get(&level)
            .expect("Could not get styles")
            .clone();
    }

    pub fn sync(&self) {
        loop {
            {
                if FILE_BUFFER
                    .read()
                    .expect("Could not get file buffer for sync")
                    .clone()
                    .is_empty()
                    && TERMINAL_BUFFER
                        .read()
                        .expect("Could not get terminal buffer for sync")
                        .clone()
                        .is_empty()
                    && !LOCKED
                        .read()
                        .expect("Could not get locked for sync")
                        .clone()
                {
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

/// Represents the severity level of a log message.
///
/// # Variants
///
/// - `Trace`: Used for standard output - lowest level.
/// - `Debug`: Used for debug messages -- will not compile for release builds.
/// - `Info`: Used for informational messages.
/// - `Warning`: Used for warning messages.
/// - `Error`: Used for error messages.
/// - `Critical`: Used for critical error messages that might require immediate attention.
/// - `Diagnostic`: Used to bypass all filtering -- will not compile for release builds.
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
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
    Critical = 5,
    Fatal = 6,
    Diagnostic = 245,
    None = 255,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Level::Trace => write!(f, "TRACE     "),
            Level::Debug => write!(f, "DEBUG     "),
            Level::Info => write!(f, "INFO      "),
            Level::Warning => write!(f, "WARNING   "),
            Level::Error => write!(f, "ERROR     "),
            Level::Critical => write!(f, "CRITICAL  "),
            Level::Fatal => write!(f, "FATAL     "),
            Level::Diagnostic => write!(f, "DIAGNOSTIC"),
            Level::None => write!(f, "NONE      "),
        }
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TimeZone {
    Local,
    Utc,
}

// ##################################################################### Test #####################################################################

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_filtering() {
        let logger = Logger::new().level(Level::Error);

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
        let logger = Logger::new().level(Level::None);

        //Test levels below
        assert!(Level::Info < logger.output_level);
        assert!(Level::Debug < logger.output_level);
        assert!(Level::Warning < logger.output_level);
        assert!(Level::Error < logger.output_level);
        assert!(Level::Critical < logger.output_level);
    }

    #[test]
    fn test_log_format() {
        let logger = Logger::new().log_format("{level} - {message}");

        let formatted_message = logger
            .log_format
            .replace("{level}", "INFO")
            .replace("{message}", "Test message");

        assert_eq!(formatted_message, "INFO - Test message");
    }

    #[test]
    fn test_output_enablement() {
        let mut logger = Logger::new();

        // Initially, terminal output is enabled, and file output is disabled.
        assert!(
            logger.terminal_output,
            "Terminal output should be enabled by default"
        );
        assert!(
            !logger.file_output,
            "File output should be disabled by default"
        );

        // Enable file output and disable terminal output.
        logger = logger.file(true).terminal(false);

        assert!(logger.file_output, "File output was not enabled");
        assert!(!logger.terminal_output, "Terminal output was not disabled");
    }

    #[test]
    fn test_ignore_levels() {
        let logger = Logger::new().ignore(Level::Debug).ignore(Level::Warning);

        assert!(
            logger.ignore.contains(&Level::Debug),
            "Debug level should be ignored"
        );
        assert!(
            logger.ignore.contains(&Level::Warning),
            "Warning level should be ignored"
        );
        assert!(
            !logger.ignore.contains(&Level::Error),
            "Error level should not be ignored"
        );
    }

    #[test]
    fn test_style_assignment() {
        let logger = Logger::new().style(Level::Info, vec![Style::FGGreen, Style::Bold]);

        let styles = logger.styles(Level::Info);
        assert!(
            styles.contains(&Style::FGGreen) && styles.contains(&Style::Bold),
            "Info level should have green and bold styles"
        );
    }
}
