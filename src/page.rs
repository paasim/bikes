use crate::err::{AppError, Res};
use crate::html_template::HtmlTemplate;
use crate::station::{Group, Station};
use crate::tile::Tile;
use askama::Template;
use axum::response::IntoResponse;
use axum::response::Response;

#[derive(Template)]
#[template(path = "layout.html")]
pub struct Page {
    groups: Vec<Group>,
    data: PageData,
}

impl Page {
    pub fn mk_response(groups: Vec<Group>, data: PageData) -> Response {
        HtmlTemplate(Self { groups, data }).into_response()
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
    pub fn with_data(stations: Vec<Station>) -> Res<Self> {
        let ref_point = Tile::from_lat_lons(stations.iter().map(|s| (s.lat, s.lon))).ok_or(
            AppError::from("Can't find a zoom level that covers the stations"),
        )?;
        Ok(Self::Data {
            stations,
            ref_point,
            pixels: 350,
        })
    }
}
