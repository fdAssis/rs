/**
 *  Your goal is to emit a log message as follows:
 *  "[<LEVEL>]: <MESSAGE>".
 *  You'll need to implement functions that correspond with log levels.
 *
 *  Exercise link: https://exercism.org/tracks/rust/exercises/semi-structured-logs
 */

#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
    }
}
pub fn info(message: &str) -> String {
    format!("[INFO]: {}", message)
}
pub fn warn(message: &str) -> String {
    format!("[WARNING]: {}", message)
}
pub fn error(message: &str) -> String {
    format!("[ERROR]: {}", message)
}

pub fn main() {
    println!("{}", log(LogLevel::Error, "Stack overflow"));
}
