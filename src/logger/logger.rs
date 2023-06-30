use colored::*;

pub struct Logger {
}

impl Logger {
    pub fn info(location: String, message: String) {
        println!("[{} | {}]: {}", "INFO".bold().blue(), location.bold().yellow(), message);
    }

    pub fn error(location: String, message: String) {
        println!("[{} | {}]: {}", "ERROR".bold().red(), location.bold().yellow(), message);
    }

    pub fn assert(expression: bool, location: String, message: String) {
        if !expression {
            println!("[{} | {}]: {}", "ERROR".bold().red(), location.bold().yellow(), message);
        }
    }

}

