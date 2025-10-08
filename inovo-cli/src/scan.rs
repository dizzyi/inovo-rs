use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};

use inovo_rs::ros_bridge::InovoRosBridge;
use roslibrust::rosbridge::{ClientHandle, ClientHandleOptions};
use tokio::task::JoinSet;
use tracing::{debug, info, instrument, trace, warn};

pub async fn scan() -> anyhow::Result<HashMap<IpAddr, Vec<(IpAddr, Result<String, String>)>>> {
    let arp = arp_all().await?;
    let filtered = filter_ros(arp).await?;
    let resolved = resolve_hostname(filtered).await?;
    Ok(resolved)
}

#[instrument(skip_all)]
pub async fn scan_command(json: bool) -> anyhow::Result<()> {
    info!("Scanning networks for PSU");
    let scan = scan().await?;

    if !json {
        return Ok(());
    }

    let map = scan
        .into_iter()
        .map(|(k, v)| {
            let mut a = vec![];
            for (i, n) in v {
                let mut o = HashMap::new();
                o.insert("ip", i.to_string());
                match n {
                    Ok(n) => {
                        o.insert("hostname", n);
                    }
                    _ => {}
                }
                a.push(o);
            }
            (k, a)
        })
        .collect::<HashMap<_, _>>();

    println!("{}", serde_json::to_string_pretty(&map)?);

    Ok(())
}

#[cfg(target_os = "windows")]
#[instrument(skip_all)]
pub async fn arp_all() -> anyhow::Result<HashMap<IpAddr, Vec<IpAddr>>> {
    info!("Executing arp command");
    let output = tokio::process::Command::new("arp")
        .arg("-a")
        .output()
        .await?
        .stdout;

    let output_string = String::from_utf8_lossy(&output).replace('\r', "");
    let mut lines = output_string.split('\n');
    let mut tables = HashMap::new();

    while let Some(l) = lines.next() {
        if l.is_empty() {
            continue;
        }
        let Some(s) = l.split_whitespace().skip(1).next() else {
            continue;
        };
        let Ok(addr) = s.parse::<std::net::IpAddr>() else {
            continue;
        };

        let mut entries = vec![];
        while let Some(l) = lines.next() {
            if l.is_empty() {
                break;
            }
            let Some(s) = l.split_whitespace().next() else {
                continue;
            };
            let Ok(addr) = s.parse::<std::net::IpAddr>() else {
                continue;
            };
            entries.push(addr);
        }
        tables.insert(addr, entries);
    }

    for (k, v) in tables.iter() {
        debug!("Interface : <{}>", k);
        for i in v {
            debug!("    {}", i);
        }
    }

    Ok(tables)
}

#[instrument(skip_all)]
pub async fn filter_ros(
    arp: HashMap<IpAddr, Vec<IpAddr>>,
) -> anyhow::Result<HashMap<IpAddr, Vec<IpAddr>>> {
    info!("Filtering ROS ip");
    let set = arp
        .into_iter()
        .map(|(k, v)| {
            let s = v
                .into_iter()
                .map(|ip| tokio::spawn(async move { ros_ping(ip).await }))
                .collect::<JoinSet<_>>();
            (k, s)
        })
        .collect::<HashMap<_, _>>();

    let mut filtered = HashMap::new();

    for (k, v) in set {
        let v = v
            .join_all()
            .await
            .into_iter()
            .filter_map(|r| match r {
                Ok(Err(_)) => None,
                Ok(Ok(ip)) => Some(ip),
                Err(e) => {
                    warn!("Joint Error : {:?}", e);
                    None
                }
            })
            .collect::<Vec<_>>();

        if v.len() > 0 {
            filtered.insert(k, v);
        }
    }

    for (k, v) in &filtered {
        debug!("Interface <{}>", k);
        for i in v {
            debug!("    {}", i)
        }
    }

    Ok(filtered)
}

#[instrument(skip_all)]
async fn ros_ping(ip: IpAddr) -> anyhow::Result<IpAddr> {
    let addr = SocketAddr::new(ip.clone(), 9090);
    let url = format!("ws://{}", addr);
    trace!("connecting rosbridge <{}>", url);
    let opts = ClientHandleOptions::new(url).timeout(std::time::Duration::from_secs(10));
    let client = ClientHandle::new_with_options(opts).await?;

    let arm_state = client.arm_state().await?;
    for i in 0..3 {
        trace!("{} - arm_state <{}> {:?}", i, ip, arm_state.next().await);
    }
    Ok(ip)
}

#[instrument(skip_all)]
async fn resolve_hostname(
    filtered: HashMap<IpAddr, Vec<IpAddr>>,
) -> anyhow::Result<HashMap<IpAddr, Vec<(IpAddr, Result<String, String>)>>> {
    info!("Filtering ROS ip");

    let set = filtered
        .into_iter()
        .map(|(k, v)| {
            let s = v
                .into_iter()
                .map(|ip| tokio::spawn(async move { ping_to_resolve(ip).await }))
                .collect::<JoinSet<_>>();
            (k, s)
        })
        .collect::<HashMap<_, _>>();

    let mut resolved = HashMap::new();

    for (k, s) in set {
        let v = s
            .join_all()
            .await
            .into_iter()
            .filter_map(|s| match s {
                Ok(Ok(ip)) => Some(ip),
                Err(e) => {
                    warn!("Joint Error : {:?}", e);
                    None
                }
                Ok(Err(e)) => {
                    warn!("Ping Error : {:?}", e);
                    None
                }
            })
            .collect::<Vec<_>>();
        resolved.insert(k, v);
    }

    for (k, v) in &resolved {
        info!("Interface <{}>", k);
        for i in v {
            info!("    {:?}", i)
        }
    }

    Ok(resolved)
}

#[cfg(target_os = "windows")]
#[instrument(skip_all)]
async fn ping_to_resolve(ip: IpAddr) -> anyhow::Result<(IpAddr, Result<String, String>)> {
    let output = tokio::process::Command::new("ping")
        .arg("-a")
        .arg("-w")
        .arg("1000")
        .arg("-n")
        .arg("1")
        .arg(ip.to_string())
        .output()
        .await?
        .stdout;

    let output_string = String::from_utf8_lossy(&output);

    if output_string.contains("timed out") {
        Ok((ip, Err("Ping Timed Out".to_string())))
    } else if output_string.contains("could not find host") {
        Ok((ip, Err("Could Not Find Host".to_string())))
    } else {
        Ok((
            ip,
            Ok(output_string
                .split_ascii_whitespace()
                .skip(1)
                .next()
                .unwrap_or_default()
                .to_owned()),
        ))
    }
}
