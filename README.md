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
logfather = "0.1.0"
```

## Usage
Here's a quick example:
```rust

use logfather::*;

fn main() {
    let mut logger = Logger::new();
    logger.file(true); // Enable file output
    logger.path("log.txt"); // Set the path for file logging

    logfather::info!("This is an info message");
}
```

## Contributing
._. why would you want to do this to me?
