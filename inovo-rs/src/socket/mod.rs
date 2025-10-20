//! Data Structure for socket communication
//!
//! # Example
//! ```no_run
//! use inovo_rs::socket::*;
//! use inovo_rs::robot::Robot;
//!
//! let mut listener = new_local_listener(50003).unwrap();
//!
//! let mut bot = listener.accept_robot().unwrap();
//! ```
use std::net::{SocketAddr, TcpListener};
use tracing::info;

use crate::robot::Robot;
use crate::util::InovorsError;

pub fn new_local_listener(port: u16) -> Result<std::net::TcpListener, InovorsError> {
    let ip = local_ip_address::local_ip()?;
    let addr = SocketAddr::from((ip, port));
    info!("creating new socket @{}", addr);
    let tcp_listener = TcpListener::bind(addr)?;
    info!("Socket binding successful.");
    Ok(tcp_listener)
}

pub trait InovoListener {
    fn accept_robot(&self) -> std::io::Result<Robot>;
}

impl InovoListener for std::net::TcpListener {
    fn accept_robot(&self) -> std::io::Result<Robot> {
        Robot::accept_from(self)
    }
}

pub mod non_blocking {
    use super::{info, InovorsError, SocketAddr};
    use tokio::net::TcpListener;

    use crate::robot::non_blocking::Robot;

    pub async fn new_local_listener(port: u16) -> Result<TcpListener, InovorsError> {
        let ip = local_ip_address::local_ip()?;
        let addr = SocketAddr::from((ip, port));
        info!("creating new socket @{}", addr);
        let tcp_listener = TcpListener::bind(addr).await?;
        info!("Socket binding successful.");
        Ok(tcp_listener)
    }

    #[async_trait::async_trait]
    pub trait InovoListener {
        async fn accept_robot(&self) -> std::io::Result<Robot>;
    }

    #[async_trait::async_trait]
    impl InovoListener for TcpListener {
        async fn accept_robot(&self) -> std::io::Result<Robot> {
            Robot::accept_from(self).await
        }
    }
}
