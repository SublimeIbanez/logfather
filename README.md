# Logfather

A simple, lightweight, and easy-to-use logging system. It allows for detailed log messages, configurable output levels, and supports both file and terminal output.

## Features
- Easy to set up and use
- Supports logging to both the terminal and log files
- Customizable log message format
- Configurable log levels (Info, Debug, Warning, Error, Critical, and Diagnostic)
- Configurable level display including colors, highlights, and styles
- Optional result (prepend `r_`) macros for managed errors
- Thread-safe

## Getting Started
To start using Logfather, add the following to your `Cargo.toml`:
```toml
[dependencies]
logfather = "0.2.6"
```
- Minimum supported Rust version: `1.61.0`
- Check out [crates.io](https://crates.io/crates/logfather)
- All the information you'll need in the [Documentation](https://docs.rs/logfather/0.2.5/logfather/)

## Usage
Macros:
- <b>Trace:</b> `trace!()` or `r_trace!()`
- <b>Debug:</b> `debug!()` or `r_debug!()`
- <b>Info:</b> `info!()` or `r_info!()`
- <b>Warning:</b> `warn!()`, `warning!()`, `r_warn!()`, or `r_warning!()`
- <b>Error:</b> `error!()` or `r_error!()`
- <b>Critical:</b> `critical!()`, `crit!()`, `r_critical!()`, or `r_crit!()`
- <b>Diagnostic:</b> `diagnostic!()`, `diag!()`, `r_diagnostic!()`, or `r_diag!()`

Quick setup for outputting to terminal:
```rust

use logfather::*;

fn main() {
    let mut logger = Logger::new(); //Terminal output is enabled by default

    error!("This is an error message");
}
```


Setting up for only file output with specific error levels to be written:
```rust

use logfather::*;

fn main() {
    let mut logger = Logger::new();
    logger.terminal(false); // Disable terminal output 
    logger.file(true); // Enable file output
    logger.path("log.txt"); // Set the path for file logging
    logger.level(Level::Error); // Set the minimum level

    info!("This is an info message"); // Will not be written to file
    debug!("This is a debug message"); // Will not be written to file
    warning!("This is a warning message"); // Will not be written to file

    error!("This is an error message"); // Will be written to file
    critical!("This is a critical message"); // Will be written to file
}
```
Set up for both terminal and file output capturing every level except warning
```rust

use logfather::*;

fn main() {
    // Supports the builder pattern
    let mut logger = Logger::new() // Terminal output is enabled by default
        .file(true) // Enable file output
        .path("log.txt") // Set the path for file logging
        .ignore(Level::Warning); // Set the specific level to ignore

    debug!("This is a debug message");
    warning!("This is a warning message"); // Will be ignored
    critical!("This is a critical message");
}
```
Handle erroneous values gracefully with the `r_` prepended macros
```rust
use logfather::*;

fn main() {
    let mut logger = Logger::new();

    match r_info!("This will return a Result<(), LogfatherError>") {
        Ok(_) => println!("Successfully logged output"),
        Err(e) => println!("Error logging output: {e}"),
    }
}
```

`Debug` and `Diagnostic` levels are Debug build only and will not be compiled in release builds
```rust

use logfather::*;

fn main() {
    debug!("This is a debug message");
    diag!("This is a diagnostic message"); 
    diagnostic!("This will not output for release builds");
}
```
## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing
._. why would you do this?
- Please open an issue with [FEATURE REQUEST] in the title if you wish to
- If you wish to contribute, issues and feature requests are a great way to do so. If you wish to fork and open a PR with changes, please provide information on what is being changed in great detail. This is my own pet project, so I will be opinionated, but I'm not against improvements or suggestions for improvements.
