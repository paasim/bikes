use crate::conf::{Conf, DtConf};
use crate::db::get_con_pool;
use crate::err::Res;
use crate::station::{get_groups, get_nearby_stations, get_stations};
use crate::tile::get_img;
use axum::extract::Request;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{Level, Span};

#[tokio::main]
pub async fn run(conf: Conf, dt_conf: DtConf) -> Res<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(Level::INFO)
        .init();
    let make_span_with = TraceLayer::new_for_http().make_span_with(default_span);
    let trace = make_span_with.on_response(log_status);

    let dt_conf = Arc::new(dt_conf);
    let pool = get_con_pool(&conf.db_url).await?;
    let app = Router::new()
        .route("/", get(get_groups))
        .with_state(pool.clone())
        .route("/stations/:name", get(get_stations))
        .route("/nearby-stations", get(get_nearby_stations))
        .route("/img", get(get_img))
        .with_state((pool, dt_conf))
        .fallback_service(ServeDir::new("static"))
        .layer(trace);
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], conf.port))).await?;

    tracing::info!("serving on {}", listener.local_addr()?);
    Ok(axum::serve(listener, app).await?)
}

fn default_span(request: &Request) -> Span {
    let request_method = request.method();
    let request_uri = request.uri();
    let request_version = request.version();
    let request_headers = request.headers();
    tracing::info_span!(
        "request",
        "method={} uri={} version={:?} headers={:?}",
        request_method,
        request_uri,
        request_version,
        request_headers
    )
}

fn log_status(response: &Response, latency: Duration, _span: &Span) {
    let stat = response.status();
    if stat.is_client_error() || stat.is_server_error() {
        tracing::error!("{} in {:?}", stat, latency)
    } else {
        tracing::info!("{} in {:?}", stat, latency)
    }
}
