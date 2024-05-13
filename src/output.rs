use crate::{
    error::*,
    logger::{OutputDirection, FILE_BUFFER, LOGGER, TERMINAL_BUFFER, TimeZone, Level}
};
use dekor::*;
use chrono::{Local, Utc};
use std::io::{stderr, stdout, BufWriter, Write};

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
    let logger;
    {
        logger = LOGGER.read().expect("Could not read logger").clone();
    }

    //If the level is too low then return
    if level < logger.output_level || logger.ignore.contains(&level) {
        return;
    }

    let message = format!("{}", args);

    //Get the time
    let time = match logger.timezone {
        TimeZone::Local => {
            let now = Local::now();
            now.format(&logger.timestamp_format).to_string()
        }
        TimeZone::Utc => {
            let now = Utc::now();
            now.format(&logger.timestamp_format).to_string()
        }
    };

    //Replace the relevant sections in the format
    let log_format = logger
        .log_format
        .replace("{timestamp}", &time)
        .replace("{module_path}", module_path)
        .replace("{message}", &message);

    //Terminal output
    if logger.terminal_output && !logger.terminal_ignore.contains(&level) {
        // Set color
        let styles = logger.styles.get(&level).unwrap();

        // Output-specific level replacement
        let format = log_format.replace("{level}", &style(styles.clone(), level));

        // Write to terminal buffer
        TERMINAL_BUFFER
            .write()
            .expect("Could not get terminal buffer")
            .push(format);
    }

    //Only write to the file if both of these are true
    if logger.file_output && !logger.file_ignore.contains(&level) && logger.file_path.is_some() {
        //Output-specific level replacement
        let format = log_format.replace("{level}", &level.to_string());

        FILE_BUFFER.write().expect("").push(format);
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
            now.format(&logger.timestamp_format).to_string()
        }
        TimeZone::Utc => {
            let now = Utc::now();
            now.format(&logger.timestamp_format).to_string()
        }
    };

    // Replace the relevant sections in the format
    let log_format = logger
        .log_format
        .replace("{timestamp}", &time)
        .replace("{module_path}", mod_path)
        .replace("{message}", &message);

    // Only write to the file if both of these are true
    if logger.file_output && !logger.file_ignore.contains(&level) {
        if let Some(mut path) = logger.file_path.clone() {
            // Handle empty path
            if path.as_os_str().is_empty() {
                // Get the current directory
                path = std::env::current_dir().map_err(LogfatherError::from)?;
                // Append default file name
                path.push(".logger");
            }

            // Check if the path contains directory separators indicating multiple directories
            if let Some(parent) = std::path::PathBuf::from(&path).parent() {
                std::fs::create_dir_all(parent).map_err(LogfatherError::from)?;
            }

            let file = std::fs::OpenOptions::new()
                .create(true)
                .read(true)
                .append(true)
                .open(&path)
                .map_err(LogfatherError::from)?;

            //Output-specific level replacement
            let format = log_format.replace("{level}", &level.to_string());

            //Lock down the file while it's being written to in case multithreaded application
            let file_mutex = std::sync::Mutex::new(file);
            {
                let mut file = file_mutex.lock().map_err(LogfatherError::from)?;
                writeln!(file, "{}", format).map_err(LogfatherError::from)?;
            }
        }
    }

    // Terminal output
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

pub(crate) fn terminal_output() {
    let mut logger;
    let mut stderr_writer = BufWriter::new(stderr());
    let mut stdout_writer = BufWriter::new(stdout());
    loop {
        {
            logger = LOGGER.read().expect("Could not read logger").clone();
        }
        match logger.terminal_std_output {
            OutputDirection::Stderr => {
                {
                    let mut terminal_buffer = TERMINAL_BUFFER
                        .write()
                        .expect("Could not output to terminal");
                    terminal_buffer
                        .iter()
                        .for_each(|line| _ = writeln!(stderr_writer, "{}", line));
                    terminal_buffer.clear();
                }
                _ = stderr_writer.flush();
            }
            OutputDirection::Stdout => {
                {
                    let mut terminal_buffer = TERMINAL_BUFFER
                        .write()
                        .expect("Could not output to terminal");
                    terminal_buffer
                        .iter()
                        .for_each(|line| _ = writeln!(stdout_writer, "{}", line));
                    terminal_buffer.clear();
                }
                _ = stdout_writer.flush();
            }
        }

        std::thread::sleep(logger.terminal_buffer_interval);
    }
}

pub(crate) fn file_output() {
    let mut logger;
    let mut buffer;
    // Use scopes to ensure dropping of lock
    loop {
        {
            logger = LOGGER.read().expect("Could not read logger").clone();
        }
        if !logger.file_output {
            std::thread::sleep(logger.file_buffer_interval);
            continue;
        }
        {
            if let Some(path) = logger.file_path {
                let file = std::fs::OpenOptions::new()
                    .create(true)
                    .read(true)
                    .append(true)
                    .open(path)
                    .expect("Could not open file");
                {
                    buffer = std::io::BufWriter::new(file);
                }
                {
                    let mut file_buffer = FILE_BUFFER
                        .write()
                        .expect("Could not read from file buffer");
                    file_buffer.iter().for_each(|line| {
                        _ = writeln!(buffer, "{}", line);
                    });
                    file_buffer.clear();
                }
                _ = buffer.flush();
            };
        }

        std::thread::sleep(logger.terminal_buffer_interval);
    }
}
