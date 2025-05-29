use colored::*;

pub struct LogHandler;

impl LogHandler {
    pub fn info(msg: &str) {
        print!("\r\x1b[2K");
        println!("{}", msg.blue());
    }
    pub fn success(msg: &str) {
        print!("\r\x1b[2K");
        println!("{}", msg.green());
    }
    pub fn warn(msg: &str) {
        print!("\r\x1b[2K");
        println!("{}", msg.yellow());
    }
    pub fn error(msg: &str) {
        print!("\r\x1b[2K");
        println!("{}", msg.red());
    }
}