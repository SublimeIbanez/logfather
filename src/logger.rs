use crate::error::*;
use chrono::{prelude::Local, Utc};
use dekor::*;
use lazy_static::lazy_static;
use simplicio::*;
use std::{io::Write, path::PathBuf};

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
/// logger.timestamp_format("%Y-%m-%d %H:%M:%S"); // Set a custom format for timestamps
/// logger.add_style(Level::Info, Style::Underline); // Set the style for INFO to Underlined in terminal output
/// ```
#[derive(Clone, Debug)]
pub struct Logger {
    pub(crate) path: Option<PathBuf>,
    pub(crate) terminal_output: bool,
    pub(crate) file_output: bool,
    pub(crate) output_level: Level,
    pub(crate) ignore: Vec<Level>,
    pub(crate) file_ignore: Vec<Level>,
    pub(crate) terminal_ignore: Vec<Level>,
    pub(crate) log_format: String,
    pub(crate) timezone: TimeZone,
    pub(crate) timestamp_format: String,
    pub(crate) styles: std::collections::HashMap<Level, Vec<Style>>,
}

impl Default for Logger {
    fn default() -> Self {
        return Self::new();
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
        Self {
            path: None,
            terminal_output: true,
            file_output: false,
            output_level: Level::Trace,
            ignore: vec![],
            file_ignore: vec![],
            terminal_ignore: vec![],
            log_format: s!("[{timestamp} {level} {module_path}] {message}"),
            timezone: TimeZone::Local,
            timestamp_format: s!("%Y-%m-%d %H:%M:%S"),
            styles: map!(
                Level::Trace        => vec![Style::FGPurple],
                Level::Debug        => vec![Style::FGBlue],
                Level::Info         => vec![Style::FGGreen],
                Level::Warning      => vec![Style::FGYellow],
                Level::Error        => vec![Style::FGRed],
                Level::Critical     => vec![Style::Bold, Style::FGRed],
                Level::Diagnostic   => vec![Style::Bold, Style::FGCyan],
                Level::None         => vec![],
            ),
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
        self.path = Some(PathBuf::from(path));
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
        self.log_format = s!(format);
        set_logger(self);
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
        set_logger(self);
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
        self.timestamp_format = s!(format);
        set_logger(self);
        return self.to_owned();
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
        self.styles.insert(level, style_set);
        return self.to_owned();
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
        let styles = self.styles.get_mut(&level).expect("Magic has occured");
        styles.push(style);
        set_logger(self);
        return self.to_owned();
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
        let styles = self.styles.get_mut(&level).expect("Magic has occured");
        styles.retain(|s| *s != style);
        set_logger(self);
        return self.to_owned();
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
        return self.styles.get(&level).expect("Magic has occured").clone();
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
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
    Critical = 5,
    Diagnostic = 245,
    None = 255,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Level::Trace => write!(f, "TRACE"),
            Level::Debug => write!(f, "DEBUG"),
            Level::Info => write!(f, "INFO"),
            Level::Warning => write!(f, "WARNING"),
            Level::Error => write!(f, "ERROR"),
            Level::Critical => write!(f, "CRITICAL"),
            Level::Diagnostic => write!(f, "DIAGNOSTIC"),
            Level::None => write!(f, "NONE"),
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

// ##################################################################### Log Definition #####################################################################

/// Logs a message with the specified log level and module path.
///
/// The log message is formatted according to the logger's configuration and output to the designated targets (file and/or terminal).
///
/// # Arguments
/// * `level` - The severity level of the log message.
/// * `module_path` - The module path where the log message originates.
/// * `message` - The log message broken into fragments.
///
/// # Examples
///
/// ``` no_run
/// use logfather::*;
///
/// // Example of manually logging an error message
/// log(Level::Error, module_path!(), format_args!("An error occurred"));
/// ```
///
/// Note: In practice, prefer using the provided macros (`info!`, `warning!`, `error!`, `critical!`) for logging.
pub fn log(level: Level, module_path: &str, args: std::fmt::Arguments) {
    //Grab a clone of the logger to not hold up any other potential logging threads
    let logger = LOGGER.read().expect("Could not read logger").clone();

    //If the level is too low then return
    if level < logger.output_level || logger.ignore.contains(&level) {
        return;
    }

    let message = format!("{}", args);

    //Get the time
    let time = match logger.timezone {
        TimeZone::Local => {
            let now = Local::now();
            s!(now.format(&logger.timestamp_format))
        }
        TimeZone::Utc => {
            let now = Utc::now();
            s!(now.format(&logger.timestamp_format))
        }
    };

    //Replace the relevant sections in the format
    let log_format = logger
        .log_format
        .replace("{timestamp}", &time)
        .replace("{module_path}", module_path)
        .replace("{message}", &message);

    //Only write to the file if both of these are true
    if logger.file_output && !logger.file_ignore.contains(&level) {
        if let Some(mut path) = logger.path {
            // Handle empty path
            if path.as_os_str().is_empty() {
                path = std::env::current_dir()
                    .unwrap_or_else(|_| panic!(
                        "{}\n{}",
                        "ERROR: No path given. Attempted to write to current directory.",
                        "  FAILURE: insufficient permissions or the current directory does not exist."
                    ));
                // Append default file name
                path.push(".logger");
            }

            // Check if the path contains directory separators indicating multiple directories
            if let Some(parent) = PathBuf::from(&path).parent() {
                std::fs::create_dir_all(parent).expect("failed to create missing sub-directories");
            }

            let file = std::fs::OpenOptions::new()
                .create(true)
                .read(true)
                .append(true)
                .open(&path);

            let file = match file {
                Ok(f) => f,
                Err(_e) => std::fs::File::create(path).expect("Could not create file"),
            };

            //Output-specific level replacement
            let format = log_format.replace("{level}", &s!(level));

            //Lock down the file while it's being written to in case multithreaded application
            let file_mutex = std::sync::Mutex::new(file);
            {
                let mut file = file_mutex.lock().unwrap();
                _ = writeln!(file, "{}", format);
            }
        }
    }

    //Terminal output
    if logger.terminal_output && !logger.terminal_ignore.contains(&level) {
        // Set color
        let styles = logger.styles.get(&level).unwrap();

        // Output-specific level replacement
        let format = log_format.replace("{level}", &style(styles.clone(), level));

        //Print to the terminal
        println!("{}", format);
    }
}

/// Logs a message with the specified log level and module path.
///
/// The log message is formatted according to the logger's configuration and output to the designated targets (file and/or terminal).
/// - Outputs a `LogfatherResult` in the event of failure instead of console outputs or panics.
///
/// # Arguments
/// * `level` - The severity level of the log message.
/// * `module_path` - The module path where the log message originates.
/// * `message` - The log message broken into fragments.
///
/// # Examples
///
/// ``` no_run
/// use logfather::*;
///
/// // Example of manually logging an error message
/// let result = result_log(Level::Error, module_path!(), format_args!("An error occurred"));
/// ```
///
/// Note: In practice, prefer using the provided macros (`info!`, `warning!`, `error!`, `critical!`) for logging.
pub fn result_log(level: Level, mod_path: &str, args: std::fmt::Arguments) -> LogfatherResult {
    //Grab a clone of the logger to not hold up any other potential logging threads
    let logger = LOGGER.read().map_err(LogfatherError::from)?.clone();

    //If the level is too low then return
    if level < logger.output_level || logger.ignore.contains(&level) {
        return Ok(());
    }

    let message = format!("{}", args);

    //Get the time
    let time = match logger.timezone {
        TimeZone::Local => {
            let now = Local::now();
            s!(now.format(&logger.timestamp_format))
        }
        TimeZone::Utc => {
            let now = Utc::now();
            s!(now.format(&logger.timestamp_format))
        }
    };

    //Replace the relevant sections in the format
    let log_format = logger
        .log_format
        .replace("{timestamp}", &time)
        .replace("{module_path}", mod_path)
        .replace("{message}", &message);

    //Only write to the file if both of these are true
    if logger.file_output && !logger.file_ignore.contains(&level) {
        if let Some(mut path) = logger.path {
            // Handle empty path
            if path.as_os_str().is_empty() {
                // Get the current directory
                path = std::env::current_dir().map_err(LogfatherError::from)?;
                // Append default file name
                path.push(".logger");
            }

            // Check if the path contains directory separators indicating multiple directories
            if let Some(parent) = PathBuf::from(&path).parent() {
                std::fs::create_dir_all(parent).map_err(LogfatherError::from)?;
            }

            let file = std::fs::OpenOptions::new()
                .create(true)
                .read(true)
                .append(true)
                .open(&path)
                .map_err(LogfatherError::from)?;

            //Output-specific level replacement
            let format = log_format.replace("{level}", &s!(level));

            //Lock down the file while it's being written to in case multithreaded application
            let file_mutex = std::sync::Mutex::new(file);
            {
                let mut file = file_mutex.lock().map_err(LogfatherError::from)?;
                writeln!(file, "{}", format).map_err(LogfatherError::from)?;
            }
        }
    }

    //Terminal output
    if logger.terminal_output && !logger.terminal_ignore.contains(&level) {
        // Set color
        let styles = logger.styles.get(&level).unwrap();

        // Output-specific level replacement
        let format = log_format.replace("{level}", &style(styles.clone(), level));

        //Print to the terminal
        println!("{}", format);
    }

    return Ok(());
}

// ##################################################################### Macro Definitions #####################################################################

/// Logs a message for tracing - very low priority.
///
/// # Example
///
/// ``` no_run
/// use logfather::trace;
///
/// trace!("This is a traced message");
/// ```
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        $crate::log($crate::Level::Trace, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a message for debugging and will be ignored on release builds.
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
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
        $crate::log($crate::Level::Debug, module_path!(), format_args!($($arg)*))
        }
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
    ($($arg:tt)*) => {{
        $crate::log($crate::Level::Info, module_path!(), format_args!($($arg)*));
    }};
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
    ($($arg:tt)*) => {{
        $crate::log($crate::Level::Warning, module_path!(), format_args!($($arg)*))
    }};
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
    ($($arg:tt)*) => {{
        $crate::log($crate::Level::Warning, module_path!(), format_args!($($arg)*))
    }};
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
    ($($arg:tt)*) => {{
        $crate::log($crate::Level::Error, module_path!(), format_args!($($arg)*))
    }};
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
    ($($arg:tt)*) => {{
        $crate::log($crate::Level::Critical, module_path!(), format_args!($($arg)*))
    }};
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
    ($($arg:tt)*) => {{
        $crate::log($crate::Level::Critical, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a diagnostic message and ignores filters -- debug builds only.
///
/// # Example
///
/// ``` no_run
/// use logfather::diagnostic;
///
/// diagnostic!("This is a critical message");
/// ```
#[macro_export]
macro_rules! diagnostic {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::log($crate::Level::Diagnostic, module_path!(), format_args!($($arg)*))
        }
    };
}

/// Logs a diagnostic message and ignores filters -- debug builds only.
///
/// # Example
///
/// ``` no_run
/// use logfather::diag;
///
/// diag!("This is a critical message");
/// ```
#[macro_export]
macro_rules! diag {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::log($crate::Level::Diagnostic, module_path!(), format_args!($($arg)*))
        }
    };
}

/// Logs a message for tracing - very low priority.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_trace;
///
/// let result = r_trace!("This is a traced message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
#[macro_export]
macro_rules! r_trace {
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Trace, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a message for debugging and will be ignored on release builds.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_debug;
///
/// let result = r_debug!("This is a debug message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
#[macro_export]
macro_rules! r_debug {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            $crate::result_log($crate::Level::Debug, module_path!(), format_args!($($arg)*))
        }
        #[cfg(not(debug_assertions))]
        {
            Ok::<(), LogfatherError>(())
        }
    }};
}

/// Logs an informational message.
///
/// # Example
///
/// ``` no_run
/// use logfather::{r_info, LogfatherError};
///
/// let result = r_info!("This is an info message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
#[macro_export]
macro_rules! r_info {
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Info, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a warning message.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_warning;
///
/// let result = r_warning!("This is a warning message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
///
/// This macro simplifies the process of logging a message at the `Warning` level.
#[macro_export]
macro_rules! r_warning {
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Warning, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a warning message.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_warn;
///
/// let result = r_warn!("This is a warning message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
///
/// This macro simplifies the process of logging a message at the `Warning` level.
#[macro_export]
macro_rules! r_warn {
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Warning, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs an error message.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_error;
///
/// let result = r_error!("This is an error message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
///
/// Use this macro for logging errors, typically when an operation fails or an unexpected condition occurs.
#[macro_export]
macro_rules! r_error {
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Error, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a critical message.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_critical;
///
/// let result = r_critical!("This is a critical message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
///
/// This macro is intended for critical errors that require immediate attention. Logging at this level typically indicates a serious failure in a component of the application.
#[macro_export]
macro_rules! r_critical {
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Critical, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a critical message.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_crit;
///
/// let result = r_crit!("This is a critical message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
///
/// This macro is intended for critical errors that require immediate attention. Logging at this level typically indicates a serious failure in a component of the application.
#[macro_export]
macro_rules! r_crit {
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Critical, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a diagnostic message and ignores filters -- debug builds only.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_diagnostic;
///
/// let result = r_diagnostic!("This is a critical message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
#[macro_export]
macro_rules! r_diagnostic {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            $crate::result_log($crate::Level::Diagnostic, module_path!(), format_args!($($arg)*))
        }
        #[cfg(not(debug_assertions))]
        {
            Ok::<(), LogfatherError>(())
        }
    }};
}

/// Logs a diagnostic message and ignores filters -- debug builds only.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_diag;
///
/// let result = r_diag!("This is a critical message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
#[macro_export]
macro_rules! r_diag {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            $crate::result_log($crate::Level::Diagnostic, module_path!(), format_args!($($arg)*))
        }
        #[cfg(not(debug_assertions))]
        {
            Ok::<(), LogfatherError>(())
        }
    }};
}
// ##################################################################### Test #####################################################################

#[cfg(test)]
mod tests {
    use super::*;

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
        logger.file(true);
        logger.terminal(false);

        assert!(logger.file_output, "File output was not enabled");
        assert!(!logger.terminal_output, "Terminal output was not disabled");
    }

    #[test]
    fn test_ignore_levels() {
        let mut logger = Logger::new();
        logger.ignore(Level::Debug);
        logger.ignore(Level::Warning);

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
        let mut logger = Logger::new();
        logger.style(Level::Info, vec![Style::FGGreen, Style::Bold]);

        let styles = logger.styles(Level::Info);
        assert!(
            styles.contains(&Style::FGGreen) && styles.contains(&Style::Bold),
            "Info level should have green and bold styles"
        );
    }
}
