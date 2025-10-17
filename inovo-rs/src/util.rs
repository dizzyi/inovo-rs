use roslibrust::rosbridge::{ClientHandle, ClientHandleOptions};

pub trait ToWsUrl {
    fn to_ws_url(self) -> String;
}

impl<T: Into<String>> ToWsUrl for T {
    fn to_ws_url(self) -> String {
        format!("ws://{}:9090", self.into())
    }
}

pub async fn rosbridge_is_alive(host: impl Into<String>, timeout: std::time::Duration) -> bool {
    ClientHandle::new_with_options(ClientHandleOptions::new(host.to_ws_url()).timeout(timeout))
        .await
        .is_ok()
}

#[cfg(feature = "scan")]
pub use scan::*;

#[cfg(feature = "scan")]
mod scan {
    use super::rosbridge_is_alive;
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
            .map(|i| i.subnet_record.into_iter().map(|i| i.psu).flatten())
            .flatten()
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
                        if !rosbridge_is_alive(ip.to_string(), SCAN_TIMEOUT).await {
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
