use crate::conf::{AppConf, DigitransitConf};
use crate::err::Result;
use crate::station::{get_group_stations, get_groups, get_nearby_stations};
use crate::tile::get_img;
use axum::Router;
use axum::extract::Request;
use axum::response::Response;
use axum::routing::get;
use std::sync::Arc;
use std::time::Duration;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{Level, Span};

#[tokio::main]
pub async fn run(app_conf: AppConf, dt_conf: DigitransitConf) -> Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(Level::INFO)
        .init();
    let make_span_with = TraceLayer::new_for_http().make_span_with(default_span);
    let trace = make_span_with.on_response(log_status);

    let dt_conf = Arc::new(dt_conf);
    let pool = app_conf.con_pool().await?;
    let app = Router::new()
        .route("/", get(get_groups))
        .with_state(pool.clone())
        .route("/stations/{name}", get(get_group_stations))
        .route("/nearby-stations", get(get_nearby_stations))
        .route("/img", get(get_img))
        .with_state((pool, dt_conf))
        .fallback_service(ServeDir::new("static"))
        .layer(trace);

    let listener = app_conf.listener().await?;
    tracing::info!("serving on {}", listener.local_addr()?);
    Ok(axum::serve(listener, app).await?)
}

fn default_span(request: &Request) -> Span {
    tracing::info_span!("request", "{} {}", request.method(), request.uri())
}

fn log_status(response: &Response, latency: Duration, _span: &Span) {
    let status = response.status();
    if status.is_client_error() || status.is_server_error() {
        tracing::error!(%status, ?latency)
    } else {
        tracing::info!(%status, ?latency)
    }
}
