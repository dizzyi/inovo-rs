use std::str::FromStr;
use std::{
    f32::consts::E, net::ToSocketAddrs
};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::{Level, debug, info, warn};

use axum::{extract::Path};
use axum::*;

use utoipa_axum::{ router::OpenApiRouter, routes};

use crate::resolve_host_to_ip;

#[derive(Debug, Default)]
pub struct Store;

pub fn route() -> OpenApiRouter {
    let store = std::sync::Arc::new(Mutex::new(Store::default()));
    OpenApiRouter::new()
        .routes(routes!(ping))
        .routes(routes!(enable_robot))
        .routes(routes!(disable_robot))
        .with_state(store)
}

#[derive(utoipa::ToSchema, Serialize, Deserialize)]
struct Pong(String);

/// Ping the Inovo API server
#[axum::debug_handler]
#[utoipa::path(get, path = "/ping", responses((status = OK, body = Pong)))]
async fn ping() -> Json<Pong> {
    info!("pinged");
    Json(Pong("Pong".to_string()))
}

#[derive(utoipa::ToSchema, Serialize, Deserialize)]
struct RobotHost(String);

#[derive(utoipa::ToSchema, Serialize, Deserialize)]
struct InovoApiResult {
    success: bool,
    message: String,
}

/// Enable robot of a given host name
#[axum::debug_handler]
#[utoipa::path(
    post, 
    path = "/enable/{host}", 
    responses(
        (status = OK, body = InovoApiResult)
    ),
    params(
    ("host" = String, Path, description = "target robot host"),
    )
)]
async fn enable_robot(Path(host): Path<String>) -> Json<InovoApiResult> {
    info!("enable robot : {}", host);
    
    // let addr = format!("{}:9090",host).to_socket_addrs().unwrap().collect::<Vec<_>>();
    // println!("{:?}", addr);

    // let Some(addr) = addr.get(1) else{
    //     return Json(InovoApiResult {
    //         success: false,
    //         message: "failed to resolve to host".to_string(),
    //     });
    // };

    let ip_addr  = match std::net::IpAddr::from_str(&host) {
        Ok(ip) => ip,
        Err(e) => {
            match resolve_host_to_ip(&host).await {
                Some(ip) => ip,
                None => {
                     return Json(InovoApiResult {
            success: false,
            message: "failed to resolve to host".to_string(),
        });
                }
            }
        }
    };
    let url = format!("ws://{}:9090", ip_addr);
        
    info!("connecting rosbridge <{}>", url);
    let opts = roslibrust::rosbridge::ClientHandleOptions::new(url)
        .timeout(std::time::Duration::from_secs(5));
    let Ok(client) = roslibrust::rosbridge::ClientHandle::new_with_options(opts).await else {
        return Json(InovoApiResult {
            success: false,
            message: "failed to connect to host".to_string(),
        });
    };
    info!("connected rosbridge");
    use inovo_rs::ros_bridge::*;

    match client.arm_enable().await {
        Ok(()) => Json(InovoApiResult {
            success: true,
            message: "successful".to_string(),
        }),
        Err(e) => Json(InovoApiResult {
            success: false,
            message: format!("unexpected error: {:?}", e),
        }),
    }
}

/// Disable robot of a given host name
#[utoipa::path(
    post, 
    path = "/disable/{host}", 
    responses(
        (status = OK, body = InovoApiResult)
    ),
    params(
    ("host" = String, Path, description = "target robot host"),
    )
)]
async fn disable_robot(Path(host): Path<String>) -> Json<InovoApiResult> {
    info!("disable robot : {}", host);


    let ip_addr  = match std::net::IpAddr::from_str(&host) {
        Ok(ip) => ip,
        Err(e) => {
            match resolve_host_to_ip(&host).await {
                Some(ip) => ip,
                None => {
                     return Json(InovoApiResult {
            success: false,
            message: "failed to resolve to host".to_string(),
        });
                }
            }
        }
    };
    
    // let addr = format!("{}:9090",host).to_socket_addrs().unwrap().collect::<Vec<_>>();
    // println!("{:?}", addr);

    // let Some(addr) = addr.get(1) else{
    //     return Json(InovoApiResult {
    //         success: false,
    //         message: "failed to resolve to host".to_string(),
    //     });
    // };
    let url = format!("ws://{}:9090", ip_addr);
        
    info!("connecting rosbridge <{}>", url);
    let opts = roslibrust::rosbridge::ClientHandleOptions::new(url)
        .timeout(std::time::Duration::from_secs(5));
    let Ok(client) = roslibrust::rosbridge::ClientHandle::new_with_options(opts).await else {
        return Json(InovoApiResult {
            success: false,
            message: "failed to connect to host".to_string(),
        });
    };
    info!("connected rosbridge");
    use inovo_rs::ros_bridge::*;

    match client.arm_disable().await {
        Ok(()) => Json(InovoApiResult {
            success: true,
            message: "successful".to_string(),
        }),
        Err(e) => Json(InovoApiResult {
            success: false,
            message: format!("unexpected error: {:?}", e),
        }),
    }
}
