use super::{Group, LocDelta, mk_stations_page};
use crate::conf::DtConf;
use crate::err::Res;
use crate::page::{Page, PageData};
use axum::extract::{Query, State};
use axum::response::Response;
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

pub async fn get_nearby_stations(
    State((pool, dt_conf)): State<(SqlitePool, Arc<DtConf>)>,
    Query(loc): Query<CurrentLocation>,
    Query(loc_d): Query<LocDelta>,
) -> Res<Response> {
    match loc.lon_lat() {
        Some(ll) => mk_stations_page(ll, loc_d, &dt_conf, &pool).await,
        None => mk_get_current_page(&pool).await,
    }
}

async fn mk_get_current_page(pool: &SqlitePool) -> Res<Response> {
    let grps = Group::get_all(pool).await?;
    Ok(Page::mk_response(grps, PageData::GetCurrent))
}
