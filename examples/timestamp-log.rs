extern crate logfather;
use logfather::*;
use chrono::{Datelike, Local};

pub enum DateStampType {
    /// used in [`get_date_stamp`] to get a String timestamp `MM-YYYY`.
    Month,
    /// used in [`get_date_stamp`] to get a String timestamp `YYYY-MM-DD`.
    Full,
}

/// Generates a date stamp based on the provided stamp type.
///
/// # Arguments
/// * `stamp_type` - The type of date stamp to generate.
///
/// # Returns
/// A string representing the date stamp, dependent on the `DateStampType` passed in.
///
/// # Examples
/// ```
/// use chrono::{Datelike, Local};
/// // returns MM-YYYY (01-2024)
/// let month_stamp = get_date_stamp(DateStampType::Month); 
/// 
/// // returns YYYY-MM-DD (2024-01-01)
/// let full_stamp = get_date_stamp(DateStampType::Full);
/// ```
fn get_date_stamp(stamp_type: DateStampType) -> String {
    let now = Local::now();

    match stamp_type {
        DateStampType::Month => format!("{:02}-{}", now.month(), now.year()),
        DateStampType::Full => format!("{}-{:02}-{:02}", now.year(), now.month(), now.day()),
    }
}

fn main() {
    let log_name = format!("log/{}{}", get_date_stamp(DateStampType::Full), ".log");

    // initialize logging
    _ = Logger::new()
        .file(true) // Enable file output
        .path(&log_name); // Set the path for file logging
   
   trace!("This is a trace message");
   critical!("This is a critical message");
}