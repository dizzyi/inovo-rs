//! Data structure for consuming structured logging message.
//!
//! The [`LoggingTarget`] trait allow the [`Logger`](crate::logger::Logger) struct to logging to multiple target.
//!
//! # Custom Logger Example
//! ```
//! use inovo_rs::logger::*;
//! pub struct OwOTarget {
//!     pub log_level: LogLevel,
//! }
//! impl target::LoggingTarget for OwOTarget {
//!     fn set_level(&mut self, log_level: LogLevel) {
//!         self.log_level = log_level
//!     }
//!     fn get_level(&self) -> LogLevel {
//!         self.log_level
//!     }
//!     fn log_message(&mut self, msg: &String, log_level: LogLevel) {
//!         let msg = msg.replace(&['r', 'l'], "w").replace(&['R', 'L'], "W");
//!         let prefix = match log_level{
//!             LogLevel::Trace => "OwO",
//!             LogLevel::Debug => "*blushes*",
//!             LogLevel::Info => "(ᗒᗨᗕ)",
//!             LogLevel::Warn => "(´,,•ω•,,)♡",
//!             _ => "*gwomps*"
//!         };
//!         print!("{:<15} : {}", prefix, msg);
//!     }
//! }
//! fn main() {
//!     // Custom Target
//!     let owo_target = OwOTarget {
//!         log_level: LogLevel::Trace,
//!     };
//!     let mut my_logger = Logger::empty().push(Box::new(owo_target));
//!     my_logger.trace("This is an example of a logger logging a message with level trace");
//!     my_logger.debug("This is an example of a logger logging a message with level debug");
//!     my_logger.info("This is an example of a logger logging a message with level info");
//!     my_logger.warn("This is an example of a logger logging a message with level warn");
//!     my_logger.error("This is an example of a logger logging a message with level error");
//! }
//! ```
//! ## Output
//! ```text
//! OwO             : Twace | This is an exampwe of a woggew wogging a message with wevew twace
//! *blushes*       : Debug | This is an exampwe of a woggew wogging a message with wevew debug
//! (ᗒᗨᗕ)           : Info  | This is an exampwe of a woggew wogging a message with wevew info
//! (´,,•ω•,,)♡     : Wawn  | This is an exampwe of a woggew wogging a message with wevew wawn
//! *gwomps*        : Ewwow | This is an exampwe of a woggew wogging a message with wevew ewwow
//! ```

use std::env;
use std::fs;
use std::io::Write;
use std::path;
use std::sync::Mutex;

use chrono;
use colored::Colorize;

use crate::logger::LogLevel;

/// A Trait for all logging target, which can set a level and log with a level
/// # Custom Logger Example
/// ```
/// use inovo_rs::logger::*;
/// pub struct OwOTarget {
///     pub log_level: LogLevel,
/// }
/// impl target::LoggingTarget for OwOTarget {
///     fn set_level(&mut self, log_level: LogLevel) {
///         self.log_level = log_level
///     }
///     fn get_level(&self) -> LogLevel {
///         self.log_level
///     }
///     fn log_message(&mut self, msg: &String, log_level: LogLevel) {
///         let msg = msg.replace(&['r', 'l'], "w").replace(&['R', 'L'], "W");
///         let prefix = match log_level{
///             LogLevel::Trace => "OwO",
///             LogLevel::Debug => "*blushes*",
///             LogLevel::Info => "(ᗒᗨᗕ)",
///             LogLevel::Warn => "(´,,•ω•,,)♡",
///             _ => "*gwomps*"
///         };
///         print!("{:<15} : {}", prefix, msg);
///     }
/// }
/// fn main() {
///     // Custom Target
///     let owo_target = OwOTarget {
///         log_level: LogLevel::Trace,
///     };
///     let mut my_logger = Logger::empty().push(Box::new(owo_target));
///     my_logger.trace("This is an example of a logger logging a message with level trace");
///     my_logger.debug("This is an example of a logger logging a message with level debug");
///     my_logger.info("This is an example of a logger logging a message with level info");
///     my_logger.warn("This is an example of a logger logging a message with level warn");
///     my_logger.error("This is an example of a logger logging a message with level error");
/// }
/// ```
/// ## Output
/// ```text
/// OwO             : Twace | This is an exampwe of a woggew wogging a message with wevew twace
/// *blushes*       : Debug | This is an exampwe of a woggew wogging a message with wevew debug
/// (ᗒᗨᗕ)           : Info  | This is an exampwe of a woggew wogging a message with wevew info
/// (´,,•ω•,,)♡     : Wawn  | This is an exampwe of a woggew wogging a message with wevew wawn
/// *gwomps*        : Ewwow | This is an exampwe of a woggew wogging a message with wevew ewwow
/// ```
pub trait LoggingTarget {
    /// set the level of the target
    fn set_level(&mut self, log_level: LogLevel);
    /// get the level of the target
    fn get_level(&self) -> LogLevel;
    /// log a message, you can implement this method without filtering message of lower log level.
    /// since it was handled.
    ///
    /// ## Parameter
    /// - `msg: &String`: the string have the log_level embedded in format of `{log_level} | {message}`
    /// - `log_level: LogLevel`: the filtering is already handled, this log level is for logging flavoring only
    fn log_message(&mut self, msg: &String, log_level: LogLevel);
    /// log a message with a level
    fn log(&mut self, msg: &String, log_level: LogLevel) {
        if self.get_level() <= log_level {
            self.log_message(msg, log_level)
        }
    }
}

/// The console logging target
///
/// ### format
/// log message to console with format of
/// - `[{name}] {log_level} | {message}`
///
/// ### color
/// different color for different level:
/// - [`LogLevel::Error`] : red
/// - [`LogLevel::Warn`] : yellow
/// - [`LogLevel::Info`] : green
/// - other : white
///
/// ### name tag padding
/// the bracketed name is padded with the maximum character of name created (min 8).
/// ```text
/// [THIS    ] Info  | a message
/// [THIS    ] Info  | ---- another message
/// [THIS IS ] Info  | a message
/// [THIS    ] Info  | ---- another message
/// [THIS IS ] Info  | ---- another message
/// [THIS IS A] Info  | a message
/// [THIS     ] Info  | ---- another message
/// [THIS IS  ] Info  | ---- another message
/// [THIS IS A] Info  | ---- another message
/// [THIS IS A LOGGER] Info  | a message
/// [THIS            ] Info  | ---- another message
/// [THIS IS         ] Info  | ---- another message
/// [THIS IS A       ] Info  | ---- another message
/// [THIS IS A LOGGER] Info  | ---- another message
///  ```
pub struct ConsoleTarget {
    name: String,
    log_level: LogLevel,
}

static PAD: Mutex<usize> = Mutex::new(8);

impl ConsoleTarget {
    /// create a console logging target, with a name and a level
    pub fn new(name: impl Into<String>, log_level: LogLevel) -> Self {
        let name = name.into();
        let mut pad = PAD.lock().unwrap();
        if name.len() > *pad {
            *pad = name.len()
        }
        Self { name, log_level }
    }
    /// create a console logging target, with a name and a default level
    pub fn default(name: impl Into<String>) -> Self {
        Self::new(name, LogLevel::Info)
    }
}

impl LoggingTarget for ConsoleTarget {
    fn log_message(&mut self, msg: &String, log_level: LogLevel) {
        let formated = format!("[{:<pad$}] {}", self.name, msg, pad = PAD.lock().unwrap());
        let colored = match log_level {
            LogLevel::Error => formated.red(),
            LogLevel::Warn => formated.yellow(),
            LogLevel::Info => formated.green(),
            _ => formated.white(),
        };
        print!("{}", colored);
    }
    fn set_level(&mut self, log_level: LogLevel) {
        self.log_level = log_level;
    }
    fn get_level(&self) -> LogLevel {
        self.log_level
    }
}

/// get the logging directory, from cargo environment variable `PATH_LOGGING`
///
/// if it is not specified, take the `./logging`
pub fn get_logging_dir() -> path::PathBuf {
    let logging_dir = option_env!("PATH_LOGGING")
        .map(|s| path::PathBuf::from(s))
        .unwrap_or(env::current_dir().unwrap().join("logging"));

    if !logging_dir.is_dir() {
        fs::create_dir(&logging_dir).unwrap();
    }

    logging_dir
}

/// get the directory of a logger, `<logging directory>/<name>/`
pub fn get_logger_dir(name: &String) -> path::PathBuf {
    let logger_dir = get_logging_dir().join(name);
    if !logger_dir.is_dir() {
        fs::create_dir(&logger_dir).unwrap();
    }
    logger_dir
}

/// The struct for rolling file logging
///
/// The logging message will be log into a file inside `<logging>/<logger name>/<logger name>.0.log`,
///
/// After the file grow to a certin size, it will trigger a rotation which will increment files sub-extension
///
/// those excess the rolling number will be discarded, and a new `<>.0.log` will be created
///
/// # Field
/// - `pub name: String`: the name of the logger
/// - `pub log_level: LogLevel`: the logging level of the logger
/// - `pub logger_dir: path::PathBuf`:the logger directory of the logger
/// - `pub trigger_size: u64`: the size of the file with will trigger rotation
/// - `pub rolling_number: u8` : the number of total file in rotation,
/// - `pub file_handle: Option<fs::File>`: the file handle of the current file,
#[derive(Debug)]
pub struct RollingFileTarget {
    name: String,
    log_level: LogLevel,
    logger_dir: path::PathBuf,
    trigger_size: u64,
    rolling_number: u8,
    file_handle: Option<fs::File>,
}

impl RollingFileTarget {
    /// create a default rolling file target with a name
    pub fn default(name: impl Into<String>) -> RollingFileTarget {
        Self::new(name, 1 << 20, 10, LogLevel::Debug)
    }
    /// create a rolling file target with a name, trigger size, rolling number, and log level
    pub fn new(
        name: impl Into<String>,
        trigger_size: u64,
        rolling_number: u8,
        log_level: LogLevel,
    ) -> RollingFileTarget {
        let name = name.into();
        let logger_dir = get_logger_dir(&name);

        let rolling_file = Self {
            name,
            log_level,
            logger_dir,
            trigger_size,
            rolling_number,
            file_handle: None,
        };
        rolling_file.rotate();
        rolling_file
    }

    /// perform rotation on the files
    pub fn rotate(&self) {
        for i in (0..self.rolling_number).rev() {
            let pathi = self._file_path(i);
            if !pathi.is_file() {
                continue;
            }
            if i == self.rolling_number - 1 {
                fs::remove_file(pathi).unwrap();
            } else {
                fs::rename(pathi, self._file_path(i + 1)).unwrap();
            }
        }
    }

    /// generate the file name of the i-th in rotation
    fn _file_path(&self, i: u8) -> path::PathBuf {
        let mut path = self.logger_dir.clone();
        path.push(format!("{}.{}.log", self.name, i));
        path
    }

    /// log a certain message
    ///
    /// before logging, check if the file already excess trigger size,
    /// perform rotation if it is
    fn _log(&mut self, msg: &String) {
        let msg = format!(
            "[{}] {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            msg
        );

        if let Some(f) = &self.file_handle {
            if f.metadata().unwrap().len() >= self.trigger_size {
                self.file_handle = None;
                self.rotate();
            }
        }

        self.file_handle = {
            let mut file = if let Some(f) = self.file_handle.take() {
                f
            } else {
                fs::File::create(self._file_path(0)).unwrap()
            };

            file.write(msg.as_bytes()).unwrap();
            file.sync_all().unwrap();

            Some(file)
        };
    }
}

impl LoggingTarget for RollingFileTarget {
    fn log_message(&mut self, msg: &String, _: LogLevel) {
        self._log(msg)
    }
    fn set_level(&mut self, log_level: LogLevel) {
        self.log_level = log_level
    }
    fn get_level(&self) -> LogLevel {
        self.log_level
    }
}
