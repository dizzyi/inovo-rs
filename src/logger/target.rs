use crate::logger::LogLevel;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path;

use chrono;

/// A Trait for all logging target, which can set a level and log with a level
pub trait LoggingTarget {
    /// log a message with a level
    fn log(&mut self, msg: &String, log_level: LogLevel) -> Result<(), String>;
    /// set the level of the target
    fn set_level(&mut self, log_level: LogLevel);
}

/// The console logging target
pub struct ConsoleTarget {
    name: String,
    log_level: LogLevel,
}

impl ConsoleTarget {
    /// create a console logging target, with a name and a level
    pub fn new(name: impl Into<String>, log_level: LogLevel) -> Self {
        Self {
            name: name.into(),
            log_level,
        }
    }
    /// create a console logging target, with a name and a default level
    pub fn default(name: impl Into<String>) -> Self {
        Self::new(name, LogLevel::Info)
    }
}

impl LoggingTarget for ConsoleTarget {
    fn log(&mut self, msg: &String, log_level: LogLevel) -> Result<(), String> {
        if self.log_level <= log_level {
            print!("[{:<25}] {}", self.name, msg);
        }
        Ok(())
    }
    fn set_level(&mut self, log_level: LogLevel) {
        self.log_level = log_level;
    }
}

/// get the logging directory, from cargo environment variable `PATH_LOGGING`
///
/// if it is not specified, take the `./logging`
pub fn get_logging_dir() -> Result<path::PathBuf, io::Error> {
    let logging_dir = option_env!("PATH_LOGGING")
        .map(|s| path::PathBuf::from(s))
        .unwrap_or(env::current_dir()?.join("logging"));

    if !logging_dir.is_dir() {
        fs::create_dir(&logging_dir)?;
    }
    Ok(logging_dir)
}

/// get the directory of a logger, `<logging directory>/<name>/`
pub fn get_logger_dir(name: &String) -> Result<path::PathBuf, io::Error> {
    let logger_dir = get_logging_dir()?.join(name);
    if !logger_dir.is_dir() {
        fs::create_dir(&logger_dir)?;
    }
    Ok(logger_dir)
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
    pub name: String,
    pub log_level: LogLevel,
    pub logger_dir: path::PathBuf,
    pub trigger_size: u64,
    pub rolling_number: u8,
    pub file_handle: Option<fs::File>,
}

impl RollingFileTarget {
    /// create a default rolling file target with a name
    pub fn default(name: impl Into<String>) -> Result<Self, io::Error> {
        Self::new(name, 1 << 20, 10, LogLevel::Debug)
    }
    /// create a rolling file target with a name, trigger size, rolling number, and log level
    pub fn new(
        name: impl Into<String>,
        trigger_size: u64,
        rolling_number: u8,
        log_level: LogLevel,
    ) -> Result<Self, io::Error> {
        let name = name.into();
        let logger_dir = get_logger_dir(&name)?;

        let rolling_file = Self {
            name,
            log_level,
            logger_dir,
            trigger_size,
            rolling_number,
            file_handle: None,
        };
        rolling_file.rotate()?;
        Ok(rolling_file)
    }

    /// perform rotation on the files
    pub fn rotate(&self) -> Result<(), io::Error> {
        for i in (0..self.rolling_number).rev() {
            let pathi = self._file_path(i);
            if !pathi.is_file() {
                continue;
            }
            if i == self.rolling_number - 1 {
                fs::remove_file(pathi)?;
            } else {
                fs::rename(pathi, self._file_path(i + 1))?;
            }
        }
        Ok(())
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
    fn _log(&mut self, msg: &String) -> Result<(), io::Error> {
        let msg = format!(
            "[{}] {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            msg
        );

        if let Some(f) = &self.file_handle {
            if f.metadata()?.len() >= self.trigger_size {
                self.file_handle = None;
                self.rotate()?;
            }
        }

        self.file_handle = {
            let mut file = if let Some(f) = self.file_handle.take() {
                f
            } else {
                fs::File::create(self._file_path(0))?
            };

            file.write(msg.as_bytes())?;
            file.sync_all()?;

            Some(file)
        };

        Ok(())
    }
}

impl LoggingTarget for RollingFileTarget {
    fn log(&mut self, msg: &String, log_level: LogLevel) -> Result<(), String> {
        if self.log_level > log_level {
            return Ok(());
        }
        match self._log(msg) {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
    fn set_level(&mut self, log_level: LogLevel) {
        self.log_level = log_level
    }
}
