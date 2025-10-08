use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::server::http::Method;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use tower_http::{
    cors::{self, AllowOrigin, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, OnRequest},
};
use tracing::{Level, Span, debug, info, info_span, span, warn};

use axum::routing::get;
use axum::*;
use axum::{extract::Request, response::IntoResponse};

use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
use utoipa_axum::{PathItemExt, router::OpenApiRouter, routes};
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

mod api;

#[derive(RustEmbed, Clone)]
#[folder = "assets/"]
struct Assets;

#[derive(OpenApi)]
#[openapi(
        tags(
            (name = "inovo-server-api", description = "Inovo Robot Arm API")
        )
    )]
struct ApiDoc;

pub async fn server(ip: Option<IpAddr>, port: u16, on_localhost: bool) -> anyhow::Result<()> {
    info!(
        "starting server : {:?} - {:?} - on-localhost {}",
        ip, port, on_localhost
    );
    let ip = match ip {
        Some(ip) => ip,
        None => {
            if on_localhost {
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
            } else {
                local_ip_address::local_ip()?
            }
        }
    };

    let trace_layer = tower_http::trace::TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new())
        .on_request(|req: &http::Request<body::Body>, span: &Span| {
            span.in_scope(|| debug!("{:>8} {}", req.method(), req.uri()));
        })
        .on_response(DefaultOnResponse::new().level(Level::DEBUG))
        .on_failure(DefaultOnFailure::new().level(Level::WARN));

    let allow_origins = AllowOrigin::list([
        "http://192.168.1.133:3000".parse().unwrap(),
        "http://localhost:3000".parse().unwrap(),
    ]);

    let cors_layer = CorsLayer::very_permissive();

    // let assets = axum_embed::ServeEmbed::<Assets>::with_parameters(
    //     None,
    //     axum_embed::FallbackBehavior::NotFound,
    //     Some("index.html".to_owned()),
    // );

    let (route, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1", api::route())
        .split_for_parts();

    let route = route
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .layer(trace_layer)
        .layer(cors_layer);

    let socket_addr = SocketAddr::new(ip, port);
    info!("binding tcp listener : {}", socket_addr);
    let listener = tokio::net::TcpListener::bind(socket_addr).await?;
    axum::serve(listener, route).await?;

    Ok(())
}
