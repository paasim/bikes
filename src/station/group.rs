use super::nearby::get_nearby;
use super::Station;
use crate::err::Res;
use crate::page::Page;
use crate::{conf::DtConf, page::PageData};
use axum::extract::State;
use axum::response::Response;
use sqlx::{query_as, SqlitePool};

pub struct Group {
    name: String,
    lon: f64,
    lat: f64,
    distance: u16,
}

impl Group {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub async fn get_all(con: &SqlitePool) -> Res<Vec<Self>> {
        let rows = query_as!(
            Self,
            r#"SELECT name, lon, lat, distance AS "distance: u16"
            FROM station_group ORDER BY name ASC"#
        )
        .fetch_all(con)
        .await?;
        Ok(rows)
    }

    pub async fn get_with_name(con: &SqlitePool, name: &str) -> Res<Self> {
        let row = query_as!(
            Self,
            r#"SELECT name, lon, lat, distance AS "distance: u16"
            FROM station_group WHERE name LIKE ?"#,
            name
        )
        .fetch_optional(con)
        .await?;
        row.ok_or(format!("No group matching the name '{}'", name).into())
    }

    pub async fn get_nearby(&self, dt_conf: &DtConf, max_results: u8) -> Res<Vec<Station>> {
        get_nearby(dt_conf, self.lon, self.lat, self.distance, max_results).await
    }
}

pub async fn get_groups(State(pool): State<SqlitePool>) -> Res<Response> {
    let groups = Group::get_all(&pool).await?;
    Ok(Page::mk_response(groups, PageData::NoData))
}
