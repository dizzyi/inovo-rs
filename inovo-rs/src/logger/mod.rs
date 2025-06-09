//! Data structure for structured logging.
//!
//! The [`Logger`] struct provide a single logging interface for multiple target.
//!
//! ```no_run
//! use inovo_rs::logger::*;
//! fn main() {
//!     // initalize a logger with default target:
//!     // - Console target with Info level
//!     // - Rolling file target with Debug level
//!     let mut logger = Logger::default_target("Logger");
//!
//!     // This message should be neither print to console nor log to file
//!     logger.trace("This is an example of a logger logging a message with level trace");
//!
//!     // This message should be log to file but not print to console
//!     logger.debug("This is an example of a logger logging a message with level debug");
//!
//!     // This Three message should be both print to console and log to file
//!     logger.info("This is an example of a logger logging a message with level info");
//!     logger.warn("This is an example of a logger logging a message with level warn");
//!     logger.error("This is an example of a logger logging a message with level error");
//! }
//! ```

pub mod target;

use target::{ConsoleTarget, LoggingTarget, RollingFileTarget};

/// Define the different level of logging
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, PartialOrd, Copy)]
pub enum LogLevel {
    Off = 5,
    Error = 4,
    Warn = 3,
    Info = 2,
    Debug = 1,
    Trace = 0,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Off => "Off",
            LogLevel::Error => "Error",
            LogLevel::Warn => "Warn",
            LogLevel::Info => "Info",
            LogLevel::Debug => "Debug",
            LogLevel::Trace => "Trace",
        }
        .to_string()
    }
}

/// The logger data structure, which contain a [`Vec`] of [`LoggingTarget`].
///
/// Allowing a single logger to perform multiple different logging action in a single call
///
/// # Example
/// this define a logger with 2 default target
/// - console logging target, level: `INFO`
/// - rolling file logging target, level : `DEBUG`
/// ```no_run
/// use inovo_rs::logger::*;
/// fn main() {
///     // initalize a logger with default target:
///     // - Console target with Info level
///     // - Rolling file target with Debug level
///     let mut logger = Logger::default_target("Logger");
///
///     // This message should be neither print to console nor log to file
///     logger.trace("This is an example of a logger logging a message with level trace");
///
///     // This message should be log to file but not print to console
///     logger.debug("This is an example of a logger logging a message with level debug");
///
///     // This Three message should be both print to console and log to file
///     logger.info("This is an example of a logger logging a message with level info");
///     logger.warn("This is an example of a logger logging a message with level warn");
///     logger.error("This is an example of a logger logging a message with level error");
/// }
/// ```
pub struct Logger {
    /// the logging target of which the logger will log to
    targets: Vec<Box<dyn LoggingTarget>>,
}

impl Logger {
    /// create a new logger given a [`Vec`] of [`LoggingTarget`]
    pub fn new(targets: Vec<Box<dyn LoggingTarget>>) -> Logger {
        Self { targets }
    }

    /// create a new logger with no target
    pub fn empty() -> Logger {
        Self::new(vec![])
    }

    /// add a new target to logger
    pub fn push(mut self, target: Box<dyn LoggingTarget>) -> Logger {
        self.targets.push(target);
        self
    }

    /// create a new logger with default target [`ConsoleTarget`] and [`RollingFileTarget`]
    /// of default logging level, with a name
    /// # Example
    /// this define a logger with 2 default target
    /// - console logging target, level: `INFO`
    /// - rolling file logging target, level : `DEBUG`
    /// # Example
    /// this define a logger with 2 default target
    /// - console logging target, level: `INFO`
    /// - rolling file logging target, level : `DEBUG`
    /// ```no_run
    /// use inovo_rs::logger::*;
    /// fn main() {
    ///     // initalize a logger with default target:
    ///     // - Console target with Info level
    ///     // - Rolling file target with Debug level
    ///     let mut logger = Logger::default_target("Logger");
    ///
    ///     // This message should be neither print to console nor log to file
    ///     logger.trace("This is an example of a logger logging a message with level trace");
    ///
    ///     // This message should be log to file but not print to console
    ///     logger.debug("This is an example of a logger logging a message with level debug");
    ///
    ///     // This Three message should be both print to console and log to file
    ///     logger.info("This is an example of a logger logging a message with level info");
    ///     logger.warn("This is an example of a logger logging a message with level warn");
    ///     logger.error("This is an example of a logger logging a message with level error");
    /// }
    /// ```
    pub fn default_target(name: impl Into<String>) -> Logger {
        Self::default_target_with_levels(name, LogLevel::Info, LogLevel::Debug)
    }

    /// create a new logger with default targets  [`ConsoleTarget`] and [`RollingFileTarget`]
    /// of specified logging level, with a name
    pub fn default_target_with_levels(
        name: impl Into<String>,
        console_log_level: LogLevel,
        file_log_level: LogLevel,
    ) -> Logger {
        let name = name.into();
        let mut console = ConsoleTarget::default(&name);
        let mut rolling_file = RollingFileTarget::default(&name);
        console.set_level(console_log_level);
        rolling_file.set_level(file_log_level);
        Self::from_console_file(console, rolling_file)
    }

    /// create a new logger with targets  [`ConsoleTarget`] and [`RollingFileTarget`]
    pub fn from_console_file(console: ConsoleTarget, rolling_file: RollingFileTarget) -> Self {
        Self {
            targets: vec![Box::new(console), Box::new(rolling_file)],
        }
    }

    /// The logging function
    ///
    /// Log the message with a specified log level,
    ///
    /// It log to all it's owned targets
    pub fn log(&mut self, msg: impl Into<String>, log_level: LogLevel) {
        let msg = format!("{:<5} | {}\n", log_level.to_string(), msg.into());
        self.targets
            .iter_mut()
            .for_each(|target| target.log(&msg, log_level));
    }

    /// log a message with level [`LogLevel::Error`]
    pub fn error(&mut self, msg: impl Into<String>) {
        self.log(msg, LogLevel::Error)
    }
    /// log a message with level [`LogLevel::Warn`]
    pub fn warn(&mut self, msg: impl Into<String>) {
        self.log(msg, LogLevel::Warn)
    }
    /// log a message with level [`LogLevel::Info`]
    pub fn info(&mut self, msg: impl Into<String>) {
        self.log(msg, LogLevel::Info)
    }
    /// log a message with level [`LogLevel::Debug`]
    pub fn debug(&mut self, msg: impl Into<String>) {
        self.log(msg, LogLevel::Debug)
    }
    /// log a message with level [`LogLevel::Trace`]
    pub fn trace(&mut self, msg: impl Into<String>) {
        self.log(msg, LogLevel::Trace)
    }
}

/// A Trait for all loggable structure
///
/// with only implementing `get_logger` function,
/// the strcut will have access to all level of logging function
pub trait Logable {
    /// get logger of the struct
    fn get_logger(&mut self) -> &mut Logger;

    fn log(&mut self, msg: impl Into<String>, log_level: LogLevel) {
        self.get_logger().log(msg, log_level)
    }

    /// log a message with level [`LogLevel::Error`]
    fn error(&mut self, msg: impl Into<String>) {
        self.log(msg, LogLevel::Error)
    }

    /// log a message with level [`LogLevel::Warn`]
    fn warn(&mut self, msg: impl Into<String>) {
        self.log(msg, LogLevel::Warn)
    }

    /// log a message with level [`LogLevel::Info`]
    fn info(&mut self, msg: impl Into<String>) {
        self.log(msg, LogLevel::Info)
    }

    /// log a message with level [`LogLevel::Debug`]
    fn debug(&mut self, msg: impl Into<String>) {
        self.log(msg, LogLevel::Debug)
    }

    /// log a message with level [`LogLevel::Trace`]
    fn trace(&mut self, msg: impl Into<String>) {
        self.log(msg, LogLevel::Trace)
    }
}

impl Logable for &mut Logger {
    fn get_logger(&mut self) -> &mut Logger {
        self
    }
}

unsafe impl Send for Logger {}
