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

use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use tracing::{debug, info};

#[derive(Debug, thiserror::Error)]
pub enum LocalListenerError {
    #[error(transparent)]
    LocalIPError(#[from] local_ip_address::Error),
    #[error(transparent)]
    StdIOError(#[from] std::io::Error),
}

pub fn new_local_listener(port: u16) -> Result<std::net::TcpListener, LocalListenerError> {
    let ip = local_ip_address::local_ip()?;
    let addr = SocketAddr::from((ip, port));
    info!("creating new socket . . .");
    info!("--- Address : {}", addr);
    let tcp_listener = TcpListener::bind(addr)?;
    info!("Socket binding successful.");
    Ok(tcp_listener)
}

pub trait InovoListener {
    fn accept_stream(&self) -> std::io::Result<InovoStream>;
}

impl InovoListener for std::net::TcpListener {
    fn accept_stream(&self) -> std::io::Result<InovoStream> {
        InovoStream::accept_from(self)
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
pub struct InovoStream {
    /// Writer to the tcp stream
    buf_writer: BufWriter<TcpStream>,
    /// Reader of the tcp stream
    buf_reader: BufReader<TcpStream>,
    /// Buffer for reading message
    buffer: String,
}

impl InovoStream {
    /// create a new stream
    ///
    /// ## Argument
    /// - `tcp_stream : TcpStream` : inner tcp stream connection
    pub fn new(tcp_stream: TcpStream) -> std::io::Result<Self> {
        let buf_writer = BufWriter::new(tcp_stream.try_clone()?);
        let buf_reader = BufReader::new(tcp_stream.try_clone()?);
        let buffer = String::new();

        info!("New Tcp Stream created successful.");

        Ok(InovoStream {
            buf_writer,
            buf_reader,
            buffer,
        })
    }
    /// connect to a socket
    ///
    /// ## Argument
    /// - `addr: SocketAddr` : target's socket address
    pub fn connect(addr: SocketAddr) -> Result<Self, io::Error> {
        Self::new(TcpStream::connect(addr)?)
    }

    pub fn accept_from(listener: &std::net::TcpListener) -> std::io::Result<Self> {
        let (conn, ip) = listener.accept()?;
        info!("Accept connection from : {ip}");
        Ok(Self::new(conn)?)
    }

    /// write a message ends with `\r\n` to the socket stream
    pub fn write(&mut self, msg: impl Into<String>) -> Result<(), io::Error> {
        let msg: String = format!("{}\r\n", msg.into());
        debug!(">>> {}", msg.trim());
        self.buf_writer.write(msg.as_bytes())?;
        self.buf_writer.flush()?;
        Ok(())
    }

    /// read a message ends with `\n` from the socket stream
    pub fn read(&mut self) -> Result<String, io::Error> {
        self.buffer.clear();
        let size = self.buf_reader.read_line(&mut self.buffer)?;
        if size == 0 {
            // return Err(std::io::Error::other("0 input bytes, diconnected"));
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "0 input bytes, disconnected",
            ));
        }
        let msg = self.buffer.clone().trim().to_string();
        debug!("<<< {}", msg);
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
