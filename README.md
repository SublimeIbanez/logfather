# Logfather

A simple, lightweight, and easy-to-use logging system. It allows for detailed log messages, configurable output levels, and supports both file and terminal output.

## Features
- Easy to set up and use
- Supports logging to both the terminal and log files
- Customizable log message format
- Configurable log levels (Info, Debug, Warning, Error, and Critical)
- Configurable level display including colors, highlights, and styles
- Thread-safe

## Getting Started
To start using Logfather, add the following to your `Cargo.toml`:
```toml
[dependencies]
logfather = "0.2.3"
```
- Minimum supported Rust version: `1.60.0`

## Usage
Macros:
- <b>Trace:</b> `trace!()`
- <b>Debug:</b> `debug!()` [`dbg!()` was used but conflicts with other crates]
- <b>Info:</b> `info!()`
- <b>Warning:</b> `warn!()` or `warning!()`
- <b>Error:</b> `error!()`
- <b>Critical:</b> `critical!()` or `crit!()`

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
    logger.terminal(false) // Disable terminal output 
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

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing
._. why would you do this?
