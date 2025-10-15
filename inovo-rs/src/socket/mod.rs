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
use tracing::{debug, info};

use crate::robot::Robot;

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

pub mod tokio {
    use super::{info, LocalListenerError, SocketAddr};
    use tokio::net::TcpListener;

    pub async fn new_local_listener(port: u16) -> Result<TcpListener, LocalListenerError> {
        let ip = local_ip_address::local_ip()?;
        let addr = SocketAddr::from((ip, port));
        info!("creating new socket @{}", addr);
        let tcp_listener = TcpListener::bind(addr).await?;
        info!("Socket binding successful.");
        Ok(tcp_listener)
    }

    #[async_trait::async_trait]
    pub trait InovoListener {
        async fn accept_robot(&self) -> std::io::Result<()>;
    }

    #[async_trait::async_trait]
    impl InovoListener for TcpListener {
        async fn accept_robot(&self) -> std::io::Result<()> {
            todo!()
        }
    }
}
