use crate::err::Result;
use crate::err_to_resp;
use crate::station::{Group, Station, StationData};
use crate::tile::Tile;
use askama::Template;
use axum::response::Response;
use axum::response::{Html, IntoResponse};

/// For rendering the data as HTML
#[derive(Template)]
#[template(path = "layout.html")]
pub struct Page {
    groups: Vec<Group>,
    data: PageData,
}

impl Page {
    pub fn new(groups: Vec<Group>, data: PageData) -> Self {
        Self { groups, data }
    }
}

impl IntoResponse for Page {
    fn into_response(self) -> Response {
        Html(err_to_resp!(self.render())).into_response()
    }
}

/// There are three separate cases:
/// - the landing page with no data (except for the station group links that is essentially just a name and the location of the station group)
/// - page with a known location; this queries for a list of nearby stations and a tile that contains the reference point
/// - page that essentially gets location from the browser and redirects to a page with a known location
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
    /// The interesting case - construct page data from location and list of nearby stations. It turns [StationData] into a vec of [Station] that contain most importantly the distance to the given reference point.
    pub fn with_data(
        d: (i8, i8),
        lon_deg: f64,
        lat_deg: f64,
        station_data: StationData,
    ) -> Result<Self> {
        let ref_point = Tile::ref_point(15, lon_deg, lat_deg) + d;
        let pixels = 350;
        Ok(Self::Data {
            stations: station_data.into_stations(&ref_point, pixels),
            ref_point,
            pixels,
        })
    }
}
