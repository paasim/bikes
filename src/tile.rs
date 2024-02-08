use crate::conf::DtConf;
use crate::err::Res;
use axum::extract::{Query, State};
use serde::Deserialize;
use sqlx::{query, SqlitePool};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub z: u8,
}

impl Tile {
    pub fn digitransit_url(&self, img_url: &str) -> String {
        format!("{}/{}/{}/{}@2x.png", img_url, self.z, self.x, self.y,)
    }

    pub fn img_url(&self, dx: u32, dy: u32) -> String {
        format!("/img?z={}&x={}&y={}", self.z, self.x + dx, self.y + dy,)
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

    pub fn from_lat_lons<L: Iterator<Item = (f64, f64)>>(lat_lons: L) -> Option<Self> {
        let init = (180.0, 180.0, -180.0, -180.0);
        let lims = lat_lons.fold(init, |(min_lat, min_lon, max_lat, max_lon), (lat, lon)| {
            (
                std::cmp::min_by(min_lat, lat, f64::total_cmp),
                std::cmp::min_by(min_lon, lon, f64::total_cmp),
                std::cmp::max_by(max_lat, lat, f64::total_cmp),
                std::cmp::max_by(max_lon, lon, f64::total_cmp),
            )
        });
        tile_from_lims(lims)
    }
}

fn tile_from_lims((min_lat, min_lon, max_lat, max_lon): (f64, f64, f64, f64)) -> Option<Tile> {
    let tiles = 2.0;
    for z in (10..20).rev() {
        let x = f64::floor(lon_x(1 << z, min_lon));
        let y = f64::floor(lat_y(1 << z, max_lat));
        let dx = f64::floor(lon_x(1 << z, max_lon)) - x;
        let dy = f64::floor(lat_y(1 << z, min_lat)) - y;
        if f64::max(dx, dy) < tiles {
            return Some(Tile {
                x: x as u32,
                y: y as u32,
                z,
            });
        }
    }
    None
}

pub fn lon_x(n: u64, lon_deg: f64) -> f64 {
    ((lon_deg + 180.0) / 360.0) * n as f64
}

pub fn lat_y(n: u64, lat_deg: f64) -> f64 {
    let lat_rad = (lat_deg / 180.0) * std::f64::consts::PI;
    ((1.0 - lat_rad.tan().asinh() / std::f64::consts::PI) / 2.0) * n as f64
}

pub async fn get_img(
    State((pool, dt_conf)): State<(SqlitePool, Arc<DtConf>)>,
    Query(tile): Query<Tile>,
) -> Res<Vec<u8>> {
    if let Some(v) = tile.get_cached_img(&pool).await? {
        return Ok(v);
    }
    let url = tile.digitransit_url(&dt_conf.img_url);
    let req = reqwest::Client::new()
        .get(url)
        .header("digitransit-subscription-key", &dt_conf.api_key)
        .send()
        .await?;
    let b = req.bytes().await?.to_vec();
    tile.cache_img(&pool, &b).await?;
    Ok(b)
}
