use crate::logger::*;

use local_ip_address;
use net2::TcpBuilder;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};

/// get the local network IPv4 address
fn get_local_ip_address_v4() -> Result<Ipv4Addr, String> {
    match local_ip_address::local_ip() {
        Ok(ip_addr) => match ip_addr {
            IpAddr::V4(ip4) => Ok(ip4),
            _ => Err("Error: Got IPv6 address instead of IPv4".to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

/// A struct respresenting Tcp listener
/// # Example
/// ```ignore
/// let mut listener = Listener::new(50004)?;
///
/// let mut stream = listener.accept(None, None)?;
/// ```
pub struct Listener {
    /// The tcp listener
    tcp_listener: TcpListener,
    /// The logger of the tcp listener
    logger: Logger,
}

impl Logable for Listener {
    fn get_logger(&mut self) -> &mut Logger {
        &mut self.logger
    }
}

impl Listener {
    /// Create a new TCP listener, bounded to a specified port
    pub fn new(port: u16) -> Result<Self, String> {
        let ip = get_local_ip_address_v4()?;
        let addr = SocketAddrV4::new(ip, port);

        let name = format!("SL {}", addr).replace(":", "-");

        let mut logger = Logger::default_target(&name)?;

        logger.info("creating new socket . . .")?;
        logger.info(format!("Port    : {}", port))?;
        logger.info(format!("IP      : {}", ip))?;
        logger.info(format!("Address : {}", ip))?;

        let tcp_listener = TcpListener::bind(addr).map_err(|e| e.to_string())?;
        logger.info("Socket binding successful.")?;

        Ok(Self {
            tcp_listener,
            logger,
        })
    }
    /// accept a new connection and return `Stream`
    ///
    /// ## Argument
    /// - `name : Option<String>` : a name for the accepted stream, default to ip address
    /// - `logger : Option<Logger>` : a logger for the accepted stream.
    pub fn accept(
        &mut self,
        name: Option<String>,
        logger: Option<Logger>,
    ) -> Result<Stream, String> {
        self.logger.info(format!(
            "accepting new connection {:?} . . .",
            self.tcp_listener.local_addr()
        ))?;

        let (tcp_stream, _) = self.tcp_listener.accept().map_err(|e| format!("{}", e))?;

        self.logger.info("successful accept new connection.")?;
        self.logger.info(format!(
            "    {}",
            tcp_stream.peer_addr().map_err(|e| e.to_string())?
        ))?;
        Stream::new(tcp_stream, name, logger)
    }
    pub fn addr(&self) -> Result<SocketAddr, String> {
        self.tcp_listener.local_addr().map_err(|e| e.to_string())
    }
}

/// A struct respresenting TCP stream
/// # Example
/// ```ignore
/// let mut client = Stream::connect(50005, addr, None, None)?;
/// let mut i = 0;
/// loop {
///     client.write(format!("{}", i))?;
///     i += 1;
///     let _ = client.read()?;
/// }
/// ```
pub struct Stream {
    /// Local socket address
    sock_addr: SocketAddr,
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
    pub fn new(
        tcp_stream: TcpStream,
        name: Option<String>,
        logger: Option<Logger>,
    ) -> Result<Self, String> {
        let buf_writer = BufWriter::new(tcp_stream.try_clone().map_err(|e| e.to_string())?);
        let buf_reader = BufReader::new(tcp_stream.try_clone().map_err(|e| e.to_string())?);
        let buffer = String::new();

        let sock_addr = tcp_stream.peer_addr().map_err(|e| e.to_string())?;
        let addr = tcp_stream.local_addr().map_err(|e| e.to_string())?;

        let name =
            format!("SS {} - {}", addr, name.unwrap_or(sock_addr.to_string())).replace(":", "-");

        let mut logger = logger.unwrap_or(Logger::default_target(name)?);
        logger.info("New Tcp Stream created successful.")?;

        Ok(Stream {
            sock_addr,
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
    pub fn connect(
        port: u16,
        addr: SocketAddr,
        name: Option<String>,
        logger: Option<Logger>,
    ) -> Result<Self, String> {
        let ip = get_local_ip_address_v4()?;
        let local_addr = SocketAddrV4::new(ip, port);

        let tcp_stream = TcpBuilder::new_v4()
            .map_err(|e| e.to_string())?
            .bind(local_addr)
            .map_err(|e| e.to_string())?
            .connect(addr)
            .map_err(|e| e.to_string())?;

        Self::new(tcp_stream, name, logger)
    }

    /// write a message ends with `\r\n` to the socket stream
    pub fn write(&mut self, msg: impl Into<String>) -> Result<(), String> {
        let msg: String = format!("{}\r\n", msg.into());
        self.debug(format!(">>> {}", msg.trim()))?;
        self.buf_writer
            .write(msg.as_bytes())
            .map_err(|e| e.to_string())?;
        self.buf_writer.flush().map_err(|e| e.to_string())?;
        Ok(())
    }

    /// read a message ends with `\r\n` from the socket stream
    pub fn read(&mut self) -> Result<String, String> {
        self.buffer.clear();
        let size = self
            .buf_reader
            .read_line(&mut self.buffer)
            .map_err(|e| e.to_string())?;
        if size == 0 {
            return Err("0 input bytes, diconnected".to_string());
        }
        let msg = self.buffer.clone().trim().to_string();
        self.debug(format!("<<< {}", msg))?;
        Ok(msg)
    }
    /// get the local socket address of the stream
    pub fn sock_addr(&self) -> SocketAddr {
        self.sock_addr
    }
}
