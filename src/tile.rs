use crate::conf::DtConf;
use crate::err::Res;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use sqlx::{SqlitePool, query};
use std::ops::Add;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub z: u8,
}

impl Tile {
    pub fn ref_point(z: u8, lon_deg: f64, lat_deg: f64) -> Tile {
        let n = 1 << z;
        let x = (lon_x(n, lon_deg) - 0.5) as u32;
        let y = (lat_y(n, lat_deg) - 0.5) as u32;
        Self { x, y, z }
    }

    pub fn rel_coord(&self, px: u16, lon_deg: f64, lat_deg: f64) -> Option<(u16, u16)> {
        let n = 1 << self.z;
        let px = px as f64;
        let x = (lon_x(n, lon_deg) - self.x as f64) / 2.0 * px;
        let y = (lat_y(n, lat_deg) - self.y as f64) / 2.0 * px;
        if x.min(y) < 0.0 || x.max(y) > px {
            return None;
        }
        Some((x.round() as u16, y.round() as u16))
    }

    pub fn digitransit_url(&self, img_url: &str) -> String {
        format!("{}/{}/{}/{}.png", img_url, self.z, self.x, self.y)
    }

    pub fn img_url(&self, dx: u32, dy: u32) -> String {
        format!("/img?z={}&x={}&y={}", self.z, self.x + dx, self.y + dy)
    }

    pub async fn get_cached_img(&self, pool: &SqlitePool) -> Res<Option<Vec<u8>>> {
        let row = query!(
            r#"SELECT data FROM image WHERE x = ? AND y = ? AND z = ?"#,
            self.x,
            self.y,
            self.z
        )
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| r.data))
    }

    pub async fn cache_img(&self, pool: &SqlitePool, data: &[u8]) -> Res<()> {
        query!(
            r#"
            INSERT INTO image (x, y, z, data) VALUES (?, ?, ?, ?)
              ON CONFLICT(x, y, z)
              DO UPDATE SET data=excluded.data, created=unixepoch();
            "#,
            self.x,
            self.y,
            self.z,
            data
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

impl Add<(i8, i8)> for Tile {
    type Output = Self;

    fn add(mut self, (dx, dy): (i8, i8)) -> Self::Output {
        self.x += dx.max(0) as u32;
        self.x -= dx.min(0).unsigned_abs() as u32;
        self.y += dy.max(0) as u32;
        self.y -= dy.min(0).unsigned_abs() as u32;
        self
    }
}

pub fn lon_x(n: u64, lon_deg: f64) -> f64 {
    ((lon_deg + 180.0) / 360.0) * n as f64
}

pub fn lat_y(n: u64, lat_deg: f64) -> f64 {
    let lat_rad = (lat_deg / 180.0) * std::f64::consts::PI;
    (1.0 - lat_rad.tan().asinh() / std::f64::consts::PI) / 2.0 * n as f64
}

// approx 600m for zoom level 15, => diagonal is approx 850m
fn _tile_height_m(n: u64) -> f64 {
    let y = lat_y(n, 60.0) as u32;
    (y_lat(n, y) - y_lat(n, y + 1)) * 110.412 * 1000.0
}

#[allow(dead_code)] // for tests
pub fn x_lon(n: u64, x: u32) -> f64 {
    x as f64 / (n as f64) * 360.0 - 180.0
}

#[allow(dead_code)] // for tests
pub fn y_lat(n: u64, y: u32) -> f64 {
    let lat_rad = ((1.0 - y as f64 / n as f64 * 2.0) * std::f64::consts::PI)
        .sinh()
        .atan();
    lat_rad / std::f64::consts::PI * 180.0
}

async fn cached_img(pool: &SqlitePool, dt_conf: &DtConf, tile: Tile) -> Res<Vec<u8>> {
    if let Some(v) = tile.get_cached_img(pool).await? {
        return Ok(v);
    }
    let resp = dt_conf.img_request(&tile).await?;
    let b = resp.bytes().await?.to_vec();
    tile.cache_img(pool, &b).await?;
    Ok(b)
}

pub async fn get_img(
    State((pool, dt_conf)): State<(SqlitePool, Arc<DtConf>)>,
    Query(tile): Query<Tile>,
) -> Res<Response> {
    let img = cached_img(&pool, &dt_conf, tile).await?;
    let headers = [(axum::http::header::CACHE_CONTROL, "max-age=604800")];
    Ok((headers, img).into_response())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lon_x_is_inv_of_x_lon() {
        let n = 2u64.pow(15);
        let x = 18651;
        let lon = x_lon(n, x);
        let x2 = lon_x(n, lon);
        assert!(x2 as u32 == x);
    }

    #[test]
    fn lat_y_is_inv_of_y_lat() {
        let n = 2u64.pow(15);
        let y = 9487;
        let lat = y_lat(n, y);
        let y2 = lat_y(n, lat);
        assert!(y2 as u32 == y);
    }
}
