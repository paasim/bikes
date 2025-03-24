use crate::conf::DigitransitConf;
use crate::err::Result;
use crate::page::{Page, PageData};
pub use group::{Group, get_group_stations, get_groups};
pub use nearby::get_nearby_stations;
use serde::Deserialize;
use sqlx::SqlitePool;
pub use stations::StationData;

mod group;
mod nearby;
mod stations;

#[derive(Debug)]
pub struct Station {
    pub id: String,
    pub name: String,
    pub count: u16,
    pub x: u16,
    pub y: u16,
    pub distance: u16,
}

impl Station {
    /// css for placing the pin for the station on top of the tile
    pub fn pin_loc(&self) -> String {
        format!("left: {}px; top: {}px;", self.x, self.y)
    }

    /// How many bikes left? Empty / low / mid / high
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

/// Delta for the given tile from (0,0) (ie. upper left corner) tile
#[derive(Deserialize, Debug)]
pub struct LocDelta {
    dx: Option<i8>,
    dy: Option<i8>,
}

/// Get all the relevant information for a given location (nearby stations)
pub async fn mk_stations_page(
    (lon, lat): (f64, f64),
    loc_d: LocDelta,
    dt_conf: &DigitransitConf,
    pool: &SqlitePool,
) -> Result<Page> {
    let d = (loc_d.dx.unwrap_or(0), loc_d.dy.unwrap_or(0));
    let maxd = d.0.abs().max(d.1.abs()) + 1;
    let station_data =
        StationData::get(dt_conf, lon, lat, maxd as u16 * 850, (maxd + 1) as u8 * 10).await?;
    let groups = Group::get_all(pool).await?;
    let data = PageData::with_data(d, lon, lat, station_data)?;
    Ok(Page::new(groups, data))
}
