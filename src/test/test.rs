use crate::logger::logger::Logger;

pub fn status() -> bool {
    false
}

pub fn logger_test() {
    Logger::info("test/test.rs/logger_test".to_string(), "Hello world!".to_string());
    Logger::error("test/test.rs/logger_test".to_string(), "Something want wrong!".to_string());
    Logger::assert(5 == 4, "test/test.rs/logger_test".to_string(), "Something want wrong!".to_string());
}

pub fn test() {
    logger_test();
}

