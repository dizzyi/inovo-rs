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

/// The `Logger`, which contain a `vec` of `LoggingTarget`.
///
/// Allowing a single logger to perform multiple different logging action in a single call
///
/// # Example
/// ```ignore
/// let logger = Logger::default_target("LOGGER 1");
/// ```
/// this define a logger with 2 default target
/// - console logging target, level: `INFO`
/// - rolling file logging target, level : `DEBUG`
/// ```ignore
/// logger.info("This an info log.");
/// ```
/// when the above method is called
/// it will both print to the console and write to file
/// ```ignore
/// logger.debug("This a debug log.");
/// ```
/// when the above method is called
/// it will only write to file
pub struct Logger {
    /// the logging target of which the logger will log to
    targets: Vec<Box<dyn LoggingTarget>>,
}

impl Logger {
    /// create a new logger given a `vec` of `LoggingTarget`
    pub fn new(targets: Vec<Box<dyn LoggingTarget>>) -> Self {
        Self { targets }
    }

    /// create a new logger with no target
    pub fn empty() -> Self {
        Self::new(vec![])
    }

    /// add a new target to logger
    pub fn push(mut self, target: Box<dyn LoggingTarget>) -> Self {
        self.targets.push(target);
        self
    }

    /// create a new logger with default target `ConsoleTarget` and `RollingFileTarget`
    /// of default logging level, with a name
    pub fn default_target(name: impl Into<String>) -> Result<Self, String> {
        Self::default_target_with_levels(name, LogLevel::Info, LogLevel::Debug)
    }

    /// create a new logger with default target `ConsoleTarget` and `RollingFileTarget`
    /// of specified logging level, with a name
    pub fn default_target_with_levels(
        name: impl Into<String>,
        console_log_level: LogLevel,
        file_log_level: LogLevel,
    ) -> Result<Self, String> {
        let name = name.into();
        let mut console = ConsoleTarget::default(&name);
        let mut rolling_file = RollingFileTarget::default(&name).map_err(|e| format!("{}", e))?;
        console.set_level(console_log_level);
        rolling_file.set_level(file_log_level);
        Ok(Self::from_console_file(console, rolling_file))
    }

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
    pub fn log(&mut self, msg: impl Into<String>, log_level: LogLevel) -> Result<(), String> {
        let msg = format!("{:<5} | {}\n", log_level.to_string(), msg.into());
        self.targets
            .iter_mut()
            .map(|target| target.log(&msg, log_level))
            .fold(Ok(()), |acc, f| match (&f, &acc) {
                (Ok(()), _) => acc,
                (Err(_), Ok(())) => f,
                (Err(s), Err(v)) => Err(format!("{},{}", v, s)),
            })
    }

    /// log a message with level `ERROR`
    pub fn error(&mut self, msg: impl Into<String>) -> Result<(), String> {
        self.log(msg, LogLevel::Error)
    }
    /// log a message with level `WARN`
    pub fn warn(&mut self, msg: impl Into<String>) -> Result<(), String> {
        self.log(msg, LogLevel::Warn)
    }
    /// log a message with level `INFO`
    pub fn info(&mut self, msg: impl Into<String>) -> Result<(), String> {
        self.log(msg, LogLevel::Info)
    }
    /// log a message with level `DEBUG`
    pub fn debug(&mut self, msg: impl Into<String>) -> Result<(), String> {
        self.log(msg, LogLevel::Debug)
    }
    /// log a message with level `TRACE`
    pub fn trace(&mut self, msg: impl Into<String>) -> Result<(), String> {
        self.log(msg, LogLevel::Trace)
    }
}

/// A Trait for all loggable
///
/// with only implementing `get_logger` function,
/// the strcut will have access to all level of logging function
pub trait Logable {
    /// get logger of the struct
    fn get_logger(&mut self) -> &mut Logger;

    fn log(&mut self, msg: impl Into<String>, log_level: LogLevel) -> Result<(), String> {
        self.get_logger().log(msg, log_level)
    }

    /// log a message with level `ERROR`
    fn error(&mut self, msg: impl Into<String>) -> Result<(), String> {
        self.log(msg, LogLevel::Error)
    }

    /// log a message with level `WARN`
    fn warn(&mut self, msg: impl Into<String>) -> Result<(), String> {
        self.log(msg, LogLevel::Warn)
    }

    /// log a message with level `INFO`
    fn info(&mut self, msg: impl Into<String>) -> Result<(), String> {
        self.log(msg, LogLevel::Info)
    }

    /// log a message with level `DEBUG`
    fn debug(&mut self, msg: impl Into<String>) -> Result<(), String> {
        self.log(msg, LogLevel::Debug)
    }

    /// log a message with level `TRACE`
    fn trace(&mut self, msg: impl Into<String>) -> Result<(), String> {
        self.log(msg, LogLevel::Trace)
    }
}

impl Logable for &mut Logger {
    fn get_logger(&mut self) -> &mut Logger {
        self
    }
}

unsafe impl Send for Logger {}
