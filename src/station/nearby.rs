use super::{Group, Station};
use crate::conf::DtConf;
use crate::err::Res;
use crate::page::{Page, PageData};
use axum::extract::{Query, State};
use axum::response::Response;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct CurrentLocation {
    latitude: Option<f64>,
    longitude: Option<f64>,
}

pub async fn get_nearby_stations(
    State((pool, dt_conf)): State<(SqlitePool, Arc<DtConf>)>,
    Query(loc): Query<CurrentLocation>,
) -> Res<Response> {
    let groups = Group::get_all(&pool).await?;
    let data = match (loc.latitude, loc.longitude) {
        (Some(lat), Some(lon)) => get_nearby(&dt_conf, lon, lat, 1000, 10)
            .await
            .and_then(PageData::with_data)?,
        _ => PageData::GetCurrent,
    };
    Ok(Page::mk_response(groups, data))
}

pub async fn get_nearby(
    dt_conf: &DtConf,
    lon: f64,
    lat: f64,
    max_distance: u16,
    max_results: u8,
) -> Res<Vec<Station>> {
    let req = reqwest::Client::new()
        .post(&dt_conf.routing_url)
        .header(reqwest::header::CONTENT_TYPE, "application/graphql")
        .header("digitransit-subscription-key", &dt_conf.api_key)
        .body(nearest_query(lon, lat, max_distance, max_results));
    Ok(req.send().await?.json::<Stations>().await?.0)
}

fn nearest_query(lon: f64, lat: f64, max_distance: u16, max_results: u8) -> String {
    format!(
        r#"
{{
  nearest(lon: {}, lat: {}, maxDistance: {}, maxResults: {}, filterByPlaceTypes: [BICYCLE_RENT]) {{
    edges {{
      node {{
        place {{
          lat
          lon
          ...on BikeRentalStation {{
            name
            stationId
            bikesAvailable
            stationId
          }}
        }}
        distance
      }}
    }}
  }}
}}
    "#,
        lon, lat, max_distance, max_results
    )
}

pub struct Stations(Vec<Station>);

impl<'de> Deserialize<'de> for Stations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // wrappers are just used to automatically parse station
        #[derive(Deserialize)]
        struct Wrapper {
            data: Data,
        }

        #[derive(Deserialize)]
        struct Data {
            nearest: Nearest,
        }

        #[derive(Deserialize)]
        struct Nearest {
            edges: Vec<Edge>,
        }

        #[derive(Deserialize)]
        struct Edge {
            node: Node,
        }

        #[derive(Deserialize)]
        struct Node {
            place: Place,
            distance: u16,
        }

        #[derive(Deserialize)]
        struct Place {
            name: String,
            lat: f64,
            lon: f64,
            #[serde(rename = "stationId")]
            station_id: String,
            #[serde(rename = "bikesAvailable")]
            bikes_available: u16,
        }
        let edges = Wrapper::deserialize(deserializer)?.data.nearest.edges;
        let stations = edges
            .into_iter()
            .map(|e| Station {
                id: e.node.place.station_id,
                name: e.node.place.name,
                count: e.node.place.bikes_available,
                lat: e.node.place.lat,
                lon: e.node.place.lon,
                distance: e.node.distance,
            })
            .collect();
        Ok(Self(stations))
    }
}
