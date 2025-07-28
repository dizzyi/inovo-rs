use inovo_rs::ros_bridge::*;
use roslibrust::rosbridge::{ClientHandle, ClientHandleOptions, Subscriber};
use std::io::Read;
use std::net;
use std::str::FromStr;
use tracing::{debug, info, warn};

#[tracing::instrument(skip_all)]
pub async fn revive(ip: String, user: String, password: String) -> Result<(), anyhow::Error> {
    info!("Reviving PSU @ <{}>", ip);
    let ip = net::IpAddr::from_str(&ip)?;
    ssh_restart(&ip, &user, &password).await?;
    info!("delaying for rcu restart");
    for i in (1..=10).rev() {
        debug!("{} . . .", i);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    ros_start(&ip).await
}

#[tracing::instrument(skip_all)]
pub async fn ssh_restart(
    ip: &net::IpAddr,
    user: &String,
    password: &String,
) -> Result<(), anyhow::Error> {
    let addr = net::SocketAddr::new(*ip, 22);
    info!("connecting SSH <{}>", addr);

    debug!("TcpStream connecting . . .");
    let tcp = net::TcpStream::connect(addr)?;
    info!("TcpStream connected");

    debug!("session initalizing . . .");
    let mut sess = ssh2::Session::new()?;
    info!("session initalized");

    info!("tcp stream setting up");
    sess.set_tcp_stream(tcp);
    debug!("handshaking . . .");
    sess.handshake()?;
    info!("handshaked");

    debug!("authenticating . . .");
    sess.userauth_password(&user, &password)?;
    info!("authentication successful");

    debug!("establishing channel session . . .");
    let mut channel = sess.channel_session()?;
    info!("established channel session");

    debug!("Executing command . . . ");
    channel.exec("systemctl restart rcu")?;
    info!("executed command.");

    debug!("reading command output . . .");
    let mut s = String::new();
    channel.read_to_string(&mut s)?;
    info!("read command output");

    for l in s.split('\n') {
        info!(">> {}", l);
    }

    debug!("closing channel . . .");
    channel.wait_close()?;
    match channel.exit_status()? {
        0 => {
            info!("channel closed successfully");
            return Ok(());
        }
        i => {
            warn!("channel closed with exit code : {}", i);
            return Err(
                std::io::Error::other(format!("channel close with exit code : {}", i)).into(),
            );
        }
    }
}

#[tracing::instrument(skip_all)]
pub async fn ros_start(ip: &net::IpAddr) -> Result<(), anyhow::Error> {
    let addr = net::SocketAddr::new(*ip, 9090);

    let url = format!("ws://{}", addr);
    info!("connecting rosbridge <{}>", url);
    let opts = ClientHandleOptions::new(url).timeout(std::time::Duration::from_secs(5));
    let client = ClientHandle::new_with_options(opts).await?;
    info!("connected rosbridge");

    let robot_state = client.robot_state().await.unwrap();
    // let arm_state = client.arm_state().await.unwrap();
    let power_state = client.power_state().await.unwrap();
    let safe_stop_state = client.safe_stop_state().await.unwrap();
    let estop_state = client.estop_state().await.unwrap();

    while !ros_start_loop(
        &client,
        &estop_state,
        &safe_stop_state,
        &power_state,
        &robot_state,
    )
    .await?
    {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}

async fn ros_start_loop(
    client: &ClientHandle,
    estop_state: &Subscriber<InovoMessage<SafetyCircuotState>>,
    safe_stop_state: &Subscriber<InovoMessage<SafetyCircuotState>>,
    power_state: &Subscriber<InovoMessage<PowerState>>,
    robot_state: &Subscriber<InovoMessage<RobotState>>,
) -> Result<bool, anyhow::Error> {
    let e = estop_state.most_recent().await.payload;
    match (e.active, e.circuit_complete) {
        (true, true) => {
            info!("estop is activated");
        }
        (false, true) => {
            debug!("activating estop circuit . . .");
            match client.estop_reset().await {
                Ok(()) => {
                    info!("activating estop circuit successfully");
                }
                Err(e) => {
                    warn!("activating estop circuit error : {:?}", e);
                    return Ok(false);
                }
            }
        }
        (false, false) => {
            warn!("estop circuit is not completed");
            return Ok(false);
        }
        (true, false) => {
            warn!("unexpected state, estop circuit active but not completed");
            return Ok(false);
        }
    }

    let s = safe_stop_state.most_recent().await.payload;
    match (s.active, s.circuit_complete) {
        (true, true) => {
            info!("safe stop is activated");
        }
        (false, true) => {
            debug!("activating safe stop circuit . . .");
            match client.safe_stop_reset().await {
                Ok(()) => {
                    info!("activating safe stop circuit successfully");
                }
                Err(e) => {
                    warn!("activating safe stop circuit error : {:?}", e);
                    return Ok(false);
                }
            }
        }
        (false, false) => {
            warn!("safe stop circuit is not completed");
            return Ok(false);
        }
        (true, false) => {
            warn!("unexpected state, safe stop circuit active but not completed");
            return Ok(false);
        }
    }

    info!("powering on robot");
    match client.power_on().await {
        Ok(()) => {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
        Err(e) => {
            warn!("error powering robot on : {:?}", e);
        }
    }

    let p = power_state.most_recent().await.payload;
    if p.fault_code != 0 {
        warn!(
            "Power State abnormal <fault_code : {:>5}, state: {}>",
            p.fault_code, p.state
        );
        return Ok(false);
    }
    info!("power state normal");

    let r = robot_state.most_recent().await.payload;
    if !r.can_enable {
        warn!("Robot State <can_enable=false>");
        return Ok(false);
    }
    info!("arm can be enabled");

    info!("enabling robot arm");
    match client.arm_enable().await {
        Ok(()) => {}
        Err(e) => {
            warn!("Arm enable error : {:?}", e);
            return Ok(false);
        }
    }

    Ok(true)
}
