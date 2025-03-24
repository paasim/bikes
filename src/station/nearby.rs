use super::{Group, LocDelta, mk_stations_page};
use crate::conf::DigitransitConf;
use crate::err::Result;
use crate::err_to_resp;
use crate::page::{Page, PageData};
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct CurrentLocation {
    lat: Option<f64>,
    lon: Option<f64>,
}

impl CurrentLocation {
    fn lon_lat(&self) -> Option<(f64, f64)> {
        Some((self.lon?, self.lat?))
    }
}

/// Render nearby stations (given current location)
pub async fn get_nearby_stations(
    State((pool, dt_conf)): State<(SqlitePool, Arc<DigitransitConf>)>,
    Query(loc): Query<CurrentLocation>,
    Query(loc_d): Query<LocDelta>,
) -> Response {
    let page = match loc.lon_lat() {
        Some(ll) => mk_stations_page(ll, loc_d, &dt_conf, &pool).await,
        None => mk_get_current_page(&pool).await,
    };
    err_to_resp!(page).into_response()
}

async fn mk_get_current_page(pool: &SqlitePool) -> Result<Page> {
    Group::get_all(pool)
        .await
        .map(|grps| Page::new(grps, PageData::GetCurrent))
}
