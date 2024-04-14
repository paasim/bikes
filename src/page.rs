use crate::err::{AppError, Res};
use crate::station::{Group, Station, StationData};
use crate::tile::Tile;
use askama::Template;
use axum::response::Response;
use axum::response::{Html, IntoResponse};

#[derive(Template)]
#[template(path = "layout.html")]
pub struct Page {
    groups: Vec<Group>,
    data: PageData,
}

impl Page {
    pub fn mk_response(groups: Vec<Group>, data: PageData) -> Response {
        match (Self { groups, data }).render() {
            Ok(s) => Html(s).into_response(),
            Err(e) => AppError::from(e).into_response(),
        }
    }
}

pub enum PageData {
    GetCurrent,
    NoData,
    Data {
        stations: Vec<Station>,
        ref_point: Tile,
        pixels: u16,
    },
}

impl PageData {
    pub fn with_data(
        d: (i8, i8),
        lon_deg: f64,
        lat_deg: f64,
        station_data: StationData,
    ) -> Res<Self> {
        let ref_point = Tile::ref_point(15, lon_deg, lat_deg) + d;
        let pixels = 350;
        Ok(Self::Data {
            stations: station_data.into_stations(&ref_point, pixels),
            ref_point,
            pixels,
        })
    }
}
