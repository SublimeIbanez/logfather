use crate::logger::*;

/// Defines the types of errors that can occur for `Logfather`.
///
/// # Variants
/// - `LoggerAccessError(String)`: Represents an error that occurs when access to the logger is denied or fails.
/// - `FileAccessError(String)`: Indicates a problem accessing a file needed for logging.
/// - `IoError(std::io::Error)`: Encompasses general input/output errors that may occur during logging operations.
///
/// # Examples
/// Handling different kinds of errors:
///
/// ```rust
/// use logfather::*;
///
///
/// let result = result_log(Level::Info, "some_module", format_args!("Hello, world!"));
/// match result {
///     Ok(_) => println!("Logged successfully"),
///     Err(e)=> println!("Logger access error: {e}"),
/// }
/// ```
///
/// # Implements
/// - `std::fmt::Display` and `std::error::Error`.
#[derive(Debug)]
pub enum LogfatherError {
    LoggerAccessError(String),
    FileAccessError(String),
    IoError(std::io::Error),
}

impl std::fmt::Display for LogfatherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogfatherError::LoggerAccessError(err) => write!(f, "Failed to access logger: {err}"),
            LogfatherError::FileAccessError(err) => write!(f, "Failed to access file: {err}"),
            LogfatherError::IoError(err) => write!(f, "I/O Error: {err}"),
        }
    }
}

// Error
impl std::error::Error for LogfatherError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return match self {
            LogfatherError::IoError(err) => err.source(),
            _ => None,
        };
    }
}

// RwLock
impl From<std::sync::PoisonError<std::sync::RwLockReadGuard<'_, Logger>>> for LogfatherError {
    fn from(value: std::sync::PoisonError<std::sync::RwLockReadGuard<'_, Logger>>) -> Self {
        return Self::LoggerAccessError(value.to_string());
    }
}

// Mutex
impl From<std::sync::PoisonError<std::sync::MutexGuard<'_, std::fs::File>>> for LogfatherError {
    fn from(value: std::sync::PoisonError<std::sync::MutexGuard<'_, std::fs::File>>) -> Self {
        return Self::FileAccessError(value.to_string());
    }
}

// IO Errors
impl From<std::io::Error> for LogfatherError {
    fn from(value: std::io::Error) -> Self {
        return Self::IoError(value);
    }
}

/// Result type representing `Result<(), LogfatherError>`
pub type LogfatherResult = Result<(), LogfatherError>;