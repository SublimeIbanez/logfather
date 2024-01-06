# Logfather

A simple, lightweight, and easy-to-use logging system which. It allows for detailed log messages, configurable output levels, and supports both file and terminal output.

## Features
- Easy to set up and use.
- Supports logging to both the terminal and log files.
- Customizable log message format.
- Configurable log levels (Info, Warning, Error, Critical)
- Thread-safe

## Getting Started
To start using Logfather, add the following to your `Cargo.toml`:
```toml
[dependencies]
logfather = "0.1.1"
```

## Usage
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
    warning!("This is a warning message"); // Will not be written to file
    error!("This is an error message"); // Will be written to file
    critical!("This is a critical message"); // Will be written to file
}
```
Set up for both terminal and file output capturing every level
```rust

use logfather::*;

fn main() {
    let mut logger = Logger::new();
    logger.file(true); // Enable file output
    logger.path("log.txt"); // Set the path for file logging

    info!("This is an info message");
}
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing
._. why would you do this?
