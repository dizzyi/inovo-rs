/// Representing inovo-rs Error
#[derive(Debug, thiserror::Error)]
pub enum InovorsError {
    #[error(transparent)]
    SocketError(#[from] std::io::Error),
    #[error(transparent)]
    RosError(#[from] roslibrust::Error),
    #[error(transparent)]
    IvaMake(#[from] IvaMakeRequestError),
    #[error(transparent)]
    JsonSer(#[from] serde_json::Error),
    #[error(transparent)]
    LocalIPError(#[from] local_ip_address::Error),
    #[error("Response Error")]
    IvaError {
        req: Instruction,
        res: String,
        reason: String,
    },
}
pub trait ToWsUrl {
    fn to_ws_url(self) -> String;
}

impl<T: Into<String>> ToWsUrl for T {
    fn to_ws_url(self) -> String {
        format!("ws://{}:9090", self.into())
    }
}

#[cfg(feature = "scan")]
pub use scan::*;

use crate::iva::{Instruction, IvaMakeRequestError};

#[cfg(feature = "scan")]
mod scan {
    use crate::ros_bridge::rosbridge_connect;
    use netdev::prelude::*;
    use std::net::Ipv4Addr;
    use tracing::warn;

    const SCAN_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(5000);

    #[derive(Debug, Clone)]
    pub struct InterfaceRecord {
        pub interface: Interface,
        pub subnet_record: Vec<SubnetRecord>,
    }

    #[derive(Debug, Clone)]
    pub struct SubnetRecord {
        pub ipv4net: Ipv4Net,
        pub psu: Vec<(Ipv4Addr, Option<String>)>,
    }

    pub async fn scan_for_all_psu() -> Vec<(Ipv4Addr, Option<String>)> {
        let mut record = scan_all_interfaces()
            .await
            .into_iter()
            .flat_map(|i| i.subnet_record.into_iter().flat_map(|i| i.psu))
            .collect::<Vec<_>>();
        record.sort();
        record
    }

    pub async fn scan_all_interfaces() -> Vec<InterfaceRecord> {
        get_interfaces()
            .into_iter()
            .filter(|i| !i.is_loopback() && i.is_up() && i.is_running() && i.gateway.is_some())
            .map(|i| tokio::task::spawn(async { scan_interface(i).await }))
            .collect::<tokio::task::JoinSet<_>>()
            .join_all()
            .await
            .into_iter()
            .filter_map(Result::ok)
            .collect()
    }

    pub async fn scan_interface(interface: Interface) -> InterfaceRecord {
        let net_task = interface
            .ipv4
            .iter()
            .cloned()
            .map(|i| {
                let mut ip = i.trunc().addr();
                let mut join_set = tokio::task::JoinSet::new();

                while i.contains(&ip) {
                    join_set.spawn(async move {
                        if rosbridge_connect(ip.to_string(), SCAN_TIMEOUT)
                            .await
                            .is_err()
                        {
                            return None;
                        }
                        let host = tokio::task::block_in_place(move || {
                            dns_lookup::lookup_addr(&ip.into())
                        })
                        .inspect_err(|e| warn!("dns look up error : {}", e))
                        .ok();

                        Some((ip, host))
                    });

                    ip = (ip.to_bits() + 1).into();
                }

                (i, join_set)
            })
            .collect::<Vec<_>>();

        let mut subnet_record = vec![];

        for (ipv4net, mut set) in net_task {
            let mut psu = vec![];
            while let Some(j) = set.join_next().await {
                match j {
                    Err(e) => warn!("{}", e),
                    Ok(None) => {}
                    Ok(Some(p)) => psu.push(p),
                }
            }
            subnet_record.push(SubnetRecord { ipv4net, psu });
        }

        InterfaceRecord {
            interface,
            subnet_record,
        }
    }
}
