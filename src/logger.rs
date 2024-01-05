use chrono::prelude::*;
use lazy_static::lazy_static;
use std::io::Write;
use ansi_term::Color;
use fs4::FileExt;

lazy_static! {
    static ref PATH: std::sync::RwLock<Option<String>> = std::sync::RwLock::new({
        None
    });

    static ref TERMINAL: std::sync::RwLock<bool> = std::sync::RwLock::new({
        true
    });

    static ref FILE: std::sync::RwLock<bool> = std::sync::RwLock::new({
        false
    });
}

pub fn set_path(new_path: &str) {
    let mut path = PATH.write().expect("Could not set logging path");
    *path = Some(new_path.to_string());
}

pub fn set_terminal(value: bool) {
    let mut terminal = TERMINAL.write().expect("Could not set terminal output");
    *terminal = value;
}

pub fn set_file(value: bool) {
    let mut file = FILE.write().expect("Could not set file output");
    *file = value;
}

pub enum Level {
    Info,
    Warning,
    Error,
    Critical,
}

impl ToString for Level {
    fn to_string(&self) -> String {
        match self {
            Level::Info => String::from("INFO"),
            Level::Warning => String::from("WARNING"),
            Level::Error => String::from("ERROR"),
            Level::Critical => String::from("CRITICAL"),
        }
    }
}
pub fn log(level: Level, module_path: &str, message: &str) {
    let now = Local::now();
    let time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    //Make an early copy to reduce potential issues with file mutex locking PATH.read()
    let path_copy = PATH.read().expect("Could not read path").clone(); 

    match path_copy {
        Some(path) => {
            if FILE.read().expect("Could not read file").clone() {
                let mut file = std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(path.as_str())
                    .unwrap();

                file.lock_exclusive().expect("Could not lock file for logging");
                match writeln!(file, "[{} {}] {}: {}", 
                    time, 
                    module_path, 
                    level.to_string(), 
                    message
                ) {
                    Ok(_) => (),
                    Err(_) => (),
                } 
                file.unlock().expect("Could not unlock file after writing");
            }
        },
        None => (),
    }

    if TERMINAL.read().expect("Could not read terminal").clone() {
        let level_color = match level {
            Level::Info => Color::Green.normal(),
            Level::Warning => Color::Yellow.normal(),
            Level::Error => Color::Red.normal(),
            Level::Critical => Color::Red.bold(),
        };

        println!("[{} {}] {}: {}", 
            time, 
            module_path, 
            level_color.paint(level.to_string()).to_string(), 
            message
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
