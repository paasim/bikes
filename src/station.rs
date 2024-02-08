use crate::conf::DtConf;
use crate::err::Res;
use crate::page::{Page, PageData};
use crate::tile::{lat_y, lon_x, Tile};
use axum::extract::{Path, State};
use axum::response::Response;
pub use group::{get_groups, Group};
pub use nearby::get_nearby_stations;
use sqlx::SqlitePool;
use std::sync::Arc;

mod group;
mod nearby;

#[derive(Debug)]
pub struct Station {
    pub id: String,
    pub name: String,
    pub count: u16,
    pub lat: f64,
    pub lon: f64,
    pub distance: u16,
}

pub fn get_px_coord(coord: f64, min: u32, px: u16) -> f64 {
    let tiles = 2.0;
    ((coord - min as f64) * px as f64 / tiles).round()
}

impl Station {
    pub fn pin_loc(&self, pt: &Tile, pixels: u16) -> String {
        let y = get_px_coord(lat_y(1 << pt.z, self.lat), pt.y, pixels);
        let x = get_px_coord(lon_x(1 << pt.z, self.lon), pt.x, pixels);
        format!("top: {}px; left: {}px;", y, x)
    }

    pub fn count_class(&self) -> &str {
        if self.count == 0 {
            "empty"
        } else if self.count < 3 {
            "low"
        } else if self.count < 6 {
            "mid"
        } else {
            "high"
        }
    }
}

pub async fn get_stations(
    State((pool, dt_conf)): State<(SqlitePool, Arc<DtConf>)>,
    Path(grp_name): Path<String>,
) -> Res<Response> {
    let groups = Group::get_all(&pool).await?;
    let grp = Group::get_with_name(&pool, &grp_name).await?;
    let stations = grp.get_nearby(&dt_conf, 7).await?;
    let data = PageData::with_data(stations)?;
    Ok(Page::mk_response(groups, data))
}
