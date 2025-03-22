use super::LocDelta;
use super::mk_stations_page;
use crate::conf::DigitransitConf;
use crate::err::Result;
use crate::err_to_resp;
use crate::page::Page;
use crate::page::PageData;
use axum::extract::State;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::response::Response;
use sqlx::{SqlitePool, query_as};
use std::sync::Arc;

pub struct Group {
    name: String,
    lon: f64,
    lat: f64,
}

impl Group {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lon_lat(&self) -> (f64, f64) {
        (self.lon, self.lat)
    }

    pub async fn get_all(con: &SqlitePool) -> Result<Vec<Self>> {
        let rows = query_as!(
            Self,
            r#"SELECT name, lon, lat FROM station_group ORDER BY name ASC"#
        )
        .fetch_all(con)
        .await?;
        Ok(rows)
    }

    pub async fn get_with_name(con: &SqlitePool, name: &str) -> Result<Self> {
        let row = query_as!(
            Self,
            r#"SELECT name, lon, lat FROM station_group WHERE name LIKE ?"#,
            name
        )
        .fetch_optional(con)
        .await?;
        row.ok_or(format!("No group matching the name '{}'", name).into())
    }
}

pub async fn get_group_stations(
    State((pool, dt_conf)): State<(SqlitePool, Arc<DigitransitConf>)>,
    Path(grp_name): Path<String>,
    Query(loc_d): Query<LocDelta>,
) -> Response {
    let grp = err_to_resp!(Group::get_with_name(&pool, &grp_name).await);

    err_to_resp!(mk_stations_page(grp.lon_lat(), loc_d, &dt_conf, &pool).await).into_response()
}

pub async fn get_groups(State(pool): State<SqlitePool>) -> Response {
    let groups = err_to_resp!(Group::get_all(&pool).await);
    Page::new(groups, PageData::NoData).into_response()
}
