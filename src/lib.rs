#![allow(clippy::needless_return)]
//! # Logfather
//! 
//! A simple, lightweight, and easy-to-use logging system. It allows for detailed log messages, configurable output levels, and supports both file and terminal output.
//! 
//! ## Features
//! - Easy to set up and use
//! - Supports logging to both the terminal and log files
//! - Customizable log message format
//! - Configurable log levels (Info, Debug, Warning, Error, Critical, and Diagnostic)
//! - Configurable level display including colors, highlights, and styles
//! - Optional result (prepend `r_`) macros for managed errors
//! - Thread-safe
//! 
//! ## Getting Started
//! To start using Logfather, add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! logfather = "0.2.7"
//! - Check out [crates.io](https://crates.io/crates/logfather)
//! ```
//! - Minimum supported Rust version: `1.61.0`
//! 
//! ## Usage
//! Macros:
//! - <b>Trace:</b> `trace!()` or `r_trace!()`
//! - <b>Debug:</b> `debug!()` or `r_debug!()`
//! - <b>Info:</b> `info!()` or `r_info!()`
//! - <b>Warning:</b> `warn!()`, `warning!()`, `r_warn!()`, or `r_warning!()`
//! - <b>Error:</b> `error!()` or `r_error!()`
//! - <b>Critical:</b> `critical!()`, `crit!()`, `r_critical!()`, or `r_crit!()`
//! - <b>Diagnostic:</b> `diagnostic!()`, `diag!()`, `r_diagnostic!()`, or `r_diag!()`
//! 
//! Quick setup for outputting to terminal:
//! ```rust
//! 
//! use logfather::*;
//! 
//! let mut logger = Logger::new(); //Terminal output is enabled by default
//! error!("This is an error message");
//! ```
//! 
//! 
//! Setting up for only file output with specific error levels to be written:
//! ```rust
//! 
//! use logfather::*;
//! 
//! let mut logger = Logger::new();
//! logger.terminal(false); // Disable terminal output 
//! logger.file(true); // Enable file output
//! logger.path("log.txt"); // Set the path for file logging
//! logger.level(Level::Error); // Set the minimum level
//! 
//! info!("This is an info message"); // Will not be written to file
//! debug!("This is a debug message"); // Will not be written to file
//! warning!("This is a warning message"); // Will not be written to file
//! 
//! error!("This is an error message"); // Will be written to file
//! critical!("This is a critical message"); // Will be written to file
//! ```
//! Set up for both terminal and file output capturing every level except warning
//! ```rust
//! 
//! use logfather::*;
//! 
//! // Supports the builder pattern
//! let mut logger = Logger::new() // Terminal output is enabled by default
//!     .file(true) // Enable file output
//!     .path("log.txt") // Set the path for file logging
//!     .ignore(Level::Warning); // Set the specific level to ignore
//! 
//! debug!("This is a debug message");
//! warning!("This is a warning message"); // Will be ignored
//! critical!("This is a critical message");
//! ```
//! Handle erroneous values gracefully with the `r_` prepended macros
//! 
//! ```rust
//! use logfather::*;
//! 
//! let mut logger = Logger::new();
//! match r_info!("This will return a Result<(), LogfatherError>") {
//!     Ok(_) => println!("Successfully logged output"),
//!     Err(e) => println!("Error logging output: {e}"),
//! }
//! ```
//! `Debug` and `Diagnostic` levels are Debug build only and will not be compiled in release builds
//! ```rust
//! 
//! use logfather::*;
//! 
//! debug!("This is a debug message");
//! diag!("This is a diagnostic message"); 
//! diagnostic!("This will not output for release builds");
//! ```


pub mod logger;
pub mod error;
pub mod output;
pub mod macros;

pub use dekor::Style;
pub use logger::Logger;
pub use logger::Level;
pub use logger::TimeZone;
pub use logger::OutputDirection;
pub use error::LogfatherError;
pub use error::LogfatherResult;
pub use output::log;
pub use output::structured_log;
pub use output::result_log;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
