use chrono::prelude::*;
use lazy_static::lazy_static;
use std::io::Write;
use ansi_term::Color;
use fs4::FileExt;

lazy_static! {
    static ref LOGGER: std::sync::RwLock<Logger> = std::sync::RwLock::new(Logger::new());
}

pub fn set_logger(new_logger: Logger) {
    let mut logger = LOGGER.write().expect("Could not access the logger");
    *logger = new_logger;
}

#[derive(Clone)]
pub struct Logger {
    pub path: Option<String>,
    pub terminal_output: bool,
    pub file_output: bool,
    pub output_level: Level,
    pub format: String,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            path: None,
            terminal_output: true,
            file_output: false,
            output_level: Level::Info,
            format: String::from("[{timestamp} {module_path}] {level}: {message}")
        }
    }

    pub fn path(&mut self, path: &str) {
        self.path = Some(path.to_string());
    }

    pub fn terminal(&mut self, value: bool) {
        self.terminal_output = value;
    }

    pub fn file(&mut self, value: bool) {
        self.file_output = value;
    }

    pub fn level(&mut self, level: Level) {
        self.output_level = level;
    }

    pub fn format(&mut self, format: &str) {
        self.format = format.to_string();
    }
}

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

pub fn log(level: Level, module_path: &str, message: &str) {
    let logger = LOGGER.read().expect("Could not read logger").clone();

    if level < logger.output_level {
        return;
    }

    let now = Local::now();
    let time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    //Make an early copy to reduce potential issues with file mutex locking PATH.read()
    let format = logger.format
        .replace("{timestamp}", &time)
        .replace("{module_path}", module_path)
        .replace("{message}", message);

    match logger.path {
        Some(path) => {
            if logger.file_output {
                let mut file = std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(path.as_str())
                    .unwrap();

                let format = format.replace("{level}", &level.to_string());

                file.lock_exclusive().expect("Could not lock file for logging");
                match writeln!(file, "{}", 
                    format
                ) {
                    Ok(_) => (),
                    Err(_) => (),
                } 
                file.unlock().expect("Could not unlock file after writing");
            }
        },
        None => (),
    }

    if logger.terminal_output {
        let level_color = match level {
            Level::Info => Color::Green.normal(),
            Level::Warning => Color::Yellow.normal(),
            Level::Error => Color::Red.normal(),
            Level::Critical => Color::Red.bold(),
            Level::None => Color::White.normal(),
        };

        let format = format.replace("{level}", &level_color.paint(&level.to_string()));

        println!("{}", 
            format
        );
    }
}

#[macro_export]
macro_rules! info {
    ($message:expr) => {
        log(Level::Info, module_path!(), $message);
    };
}

#[macro_export]
macro_rules! warning {
    ($message:expr) => {
        log(Level::Warning, module_path!(), $message);
    };
}

#[macro_export]
macro_rules! error {
    ($message:expr) => {
        log(Level::Error, module_path!(), $message);
    };
}

#[macro_export]
macro_rules! critical {
    ($message:expr) => {
        log(Level::Critical, module_path!(), $message);
    };
}

pub use info;
pub use warning;
pub use error;
pub use critical;
