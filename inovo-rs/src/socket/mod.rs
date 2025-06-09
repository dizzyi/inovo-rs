//! Data Structure for socket communication
//!
//! # Example
//! ```no_run
//! use inovo_rs::socket::{Listener, Stream};
//!
//! let mut listener = Listener::new(50003, None).unwrap();
//!
//! let addr = listener.addr().unwrap();
//!
//! let mut client = Stream::connect(50004, addr, None).unwrap();
//!
//! let mut stream = listener.accept(None).unwrap();
//!
//! client.write("Marco").unwrap();
//! assert_eq!(stream.read().unwrap(), "Macro");
//!
//! stream.write("Polo").unwrap();
//! assert_eq!(client.read().unwrap(), "Polo");
//! ```

use net2::TcpBuilder;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

use crate::logger::*;

/// A struct respresenting Tcp listener
/// # Example
/// ```no_run
/// use inovo_rs::socket::*;
///
/// let mut listener = Listener::new(50003, None).unwrap();
///
/// let mut stream = listener.accept(None).unwrap();
/// ```
pub struct Listener {
    /// The logger of the tcp listener
    logger: Logger,
    /// The tcp listener
    tcp_listener: TcpListener,
}

impl Logable for Listener {
    fn get_logger(&mut self) -> &mut Logger {
        &mut self.logger
    }
}

impl Listener {
    /// Create a new TCP listener, bounded to a specified port
    pub fn new(port: u16, logger: Option<Logger>) -> Result<Listener, io::Error> {
        let ip = local_ip_address::local_ip().unwrap();
        let addr = SocketAddr::from((ip, port));

        let mut logger = logger.unwrap_or_else(|| {
            let name = format!("Listener {}", addr).replace(":", "-");
            Logger::default_target(&name)
        });

        logger.info("creating new socket . . .");
        logger.info(format!("--- Address : {}", addr));

        let tcp_listener = TcpListener::bind(addr)?;
        logger.info("Socket binding successful.");

        Ok(Self {
            tcp_listener,
            logger,
        })
    }
    /// accept a new connection and return `Stream`
    ///
    /// ## Argument
    /// - `logger : Option<Logger>` : a logger for the accepted stream.
    pub fn accept(&mut self, logger: Option<Logger>) -> Result<Stream, io::Error> {
        self.info("accepting new connection . . .");

        let (tcp_stream, _) = self.tcp_listener.accept()?;

        self.info("successful accept new connection.");
        self.info(format!("    {}", tcp_stream.peer_addr()?));

        let logger = logger.unwrap_or_else(|| {
            let local_addr = self
                .tcp_listener
                .local_addr()
                .unwrap()
                .to_string()
                .replace(":", "-");
            let peer_addr = tcp_stream
                .peer_addr()
                .unwrap()
                .to_string()
                .replace(":", "-");
            Logger::default_target(format!("Handle {} {}", local_addr, peer_addr))
        });

        Stream::new(tcp_stream, logger)
    }

    pub fn addr(&self) -> Result<SocketAddr, io::Error> {
        self.tcp_listener.local_addr()
    }
}

/// A struct respresenting TCP stream
/// # Example
/// ```no_run
/// use inovo_rs::socket::*;
/// use std::net::SocketAddr;
///
/// let addr = SocketAddr::from(([192,168,1,2],50003));
/// let mut client = Stream::connect(50005, addr, None).unwrap();
///
/// client.write("some string").unwrap();
/// let s: String = client.read().unwrap();
/// ```
pub struct Stream {
    /// Writer to the tcp stream
    buf_writer: BufWriter<TcpStream>,
    /// Reader of the tcp stream
    buf_reader: BufReader<TcpStream>,
    /// Buffer for reading message
    buffer: String,
    /// Logger of tcp stream
    logger: Logger,
}

impl Logable for Stream {
    fn get_logger(&mut self) -> &mut Logger {
        &mut self.logger
    }
}

impl Stream {
    /// create a new stream
    ///
    /// ## Argument
    /// - `name : Option<String>` : a name for the accepted stream, default to ip address
    /// - `logger : Option<Logger>` : a logger for the accepted stream.
    pub fn new(tcp_stream: TcpStream, mut logger: Logger) -> Result<Self, io::Error> {
        let buf_writer = BufWriter::new(tcp_stream.try_clone()?);
        let buf_reader = BufReader::new(tcp_stream.try_clone()?);
        let buffer = String::new();

        logger.info("New Tcp Stream created successful.");

        Ok(Stream {
            buf_writer,
            buf_reader,
            buffer,
            logger,
        })
    }
    /// connect to a socket
    ///
    /// ## Argument
    /// - `addr: SocketAddr` : target's socket address
    /// - `name : Option<String>` : a name for the accepted stream, default to ip address
    /// - `logger : Option<Logger>` : a logger for the accepted stream.
    pub fn connect(port: u16, addr: SocketAddr, logger: Option<Logger>) -> Result<Self, io::Error> {
        let ip = local_ip_address::local_ip().unwrap();
        let local_addr = SocketAddr::from((ip, port));

        let logger = logger.unwrap_or_else(|| {
            let peer_addr = addr.to_string().replace(":", "-");
            let local_addr = local_addr.clone().to_string().replace(":", "-");
            Logger::default_target(format!("Client {} {}", local_addr, peer_addr))
        });

        let tcp_stream = TcpBuilder::new_v4()?.bind(local_addr)?.connect(addr)?;

        Self::new(tcp_stream, logger)
    }

    /// write a message ends with `\r\n` to the socket stream
    pub fn write(&mut self, msg: impl Into<String>) -> Result<(), io::Error> {
        let msg: String = format!("{}\r\n", msg.into());
        self.debug(format!(">>> {}", msg.trim()));
        self.buf_writer.write(msg.as_bytes())?;
        self.buf_writer.flush()?;
        Ok(())
    }

    /// read a message ends with `\n` from the socket stream
    pub fn read(&mut self) -> Result<String, io::Error> {
        self.buffer.clear();
        let size = self.buf_reader.read_line(&mut self.buffer)?;
        if size == 0 {
            return Err(std::io::Error::other("0 input bytes, diconnected"));
        }
        let msg = self.buffer.clone().trim().to_string();
        self.debug(format!("<<< {}", msg));
        Ok(msg)
    }
    /// get the local socket address of the stream
    pub fn local_addr(&self) -> Result<SocketAddr, io::Error> {
        self.buf_writer.get_ref().local_addr()
    }

    /// get the peer socket address of the stream
    pub fn peer_addr(&self) -> Result<SocketAddr, io::Error> {
        self.buf_writer.get_ref().peer_addr()
    }
}
