
extern crate logfather;
use logfather::*;

fn main() {
   let mut logger = Logger::new();
   logger.terminal(false); // Disable terminal output 
   logger.file(true); // Enable file output
   logger.path("log/log.txt"); // Set the path for file logging
   logger.level(Level::Error); // Set the minimum level

   let thing = "asdf"; 
   error!("This is a trace");
   error!("this is a {thing}");
   error!("this is a {}", thing);
   error!("key1" = "value1", "key2" = "value2"; "This is also a trace");
   //error!("This is an error message"); // Will be written to file
   //critical!("This is a critical message"); // Will be written to file
}