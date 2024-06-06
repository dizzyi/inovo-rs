use inovo_rs::logger::*;

pub struct OwOTarget {
    pub log_level: LogLevel,
}

impl target::LoggingTarget for OwOTarget {
    fn set_level(&mut self, log_level: LogLevel) {
        self.log_level = log_level
    }
    fn get_level(&self) -> LogLevel {
        self.log_level
    }
    fn log_message(&mut self, msg: &String, log_level: LogLevel) {
        let msg = msg.replace(&['r', 'l'], "w").replace(&['R', 'L'], "W");
        let prefix = match log_level {
            LogLevel::Trace => "OwO",
            LogLevel::Debug => "*blushes*",
            LogLevel::Info => "(ᗒᗨᗕ)",
            LogLevel::Warn => "(,,•ω•,,)♡",
            _ => "*gwomps*",
        };
        print!("{:<15} : {}", prefix, msg);
    }
}

fn main() {
    // initalize a logger with default target:
    // - Console target with Info level
    // - Rolling file target with Debug level
    let mut logger = Logger::default_target("Logger");

    // This message should be neither print to console nor log to file
    logger.trace("This is an example of a logger logging a message with level trace");

    // This message should be log to file but not print to console
    logger.debug("This is an example of a logger logging a message with level debug");

    // This Three message should be both print to console and log to file
    logger.info("This is an example of a logger logging a message with level info");
    logger.warn("This is an example of a logger logging a message with level warn");
    logger.error("This is an example of a logger logging a message with level error");

    // Custom Target
    let owo_target = OwOTarget {
        log_level: LogLevel::Trace,
    };
    let mut my_logger = Logger::empty().push(Box::new(owo_target));

    my_logger.trace("This is an example of a logger logging a message with level trace");
    my_logger.debug("This is an example of a logger logging a message with level debug");
    my_logger.info("This is an example of a logger logging a message with level info");
    my_logger.warn("This is an example of a logger logging a message with level warn");
    my_logger.error("This is an example of a logger logging a message with level error");
}
