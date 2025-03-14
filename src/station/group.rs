use super::LocDelta;
use super::mk_stations_page;
use crate::conf::DtConf;
use crate::err::Res;
use crate::page::Page;
use crate::page::PageData;
use axum::extract::State;
use axum::extract::{Path, Query};
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

    pub async fn get_all(con: &SqlitePool) -> Res<Vec<Self>> {
        let rows = query_as!(
            Self,
            r#"SELECT name, lon, lat FROM station_group ORDER BY name ASC"#
        )
        .fetch_all(con)
        .await?;
        Ok(rows)
    }

    pub async fn get_with_name(con: &SqlitePool, name: &str) -> Res<Self> {
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
    State((pool, dt_conf)): State<(SqlitePool, Arc<DtConf>)>,
    Path(grp_name): Path<String>,
    Query(loc_d): Query<LocDelta>,
) -> Res<Response> {
    let grp = Group::get_with_name(&pool, &grp_name).await?;
    mk_stations_page(grp.lon_lat(), loc_d, &dt_conf, &pool).await
}

pub async fn get_groups(State(pool): State<SqlitePool>) -> Res<Response> {
    let groups = Group::get_all(&pool).await?;
    Ok(Page::mk_response(groups, PageData::NoData))
}
