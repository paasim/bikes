use super::Station;
use crate::conf::DigitransitConf;
use crate::err::Result;
use crate::tile::Tile;
use serde::Deserialize;

pub struct StationData(Vec<StationObs>);

#[derive(Debug)]
pub struct StationObs {
    id: String,
    name: String,
    count: u16,
    lon: f64,
    lat: f64,
    distance: u16,
}

impl StationData {
    pub async fn get(
        dt_conf: &DigitransitConf,
        lon: f64,
        lat: f64,
        max_distance: u16,
        max_results: u8,
    ) -> Result<Self> {
        let req = dt_conf.nearby_request(lon, lat, max_distance, max_results);
        Ok(req.await?.json::<StationData>().await?)
    }

    pub fn into_stations(self, ref_pt: &Tile, px: u16) -> Vec<Station> {
        self.0
            .into_iter()
            .filter_map(|s| {
                let (x, y) = ref_pt.rel_coord(px, s.lon, s.lat)?;
                Some(Station {
                    id: s.id,
                    name: s.name,
                    count: s.count,
                    x,
                    y,
                    distance: s.distance,
                })
            })
            .collect()
    }
}

impl<'de> Deserialize<'de> for StationData {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
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
            .map(|e| StationObs {
                id: e.node.place.station_id,
                name: e.node.place.name,
                count: e.node.place.bikes_available,
                lon: e.node.place.lon,
                lat: e.node.place.lat,
                distance: e.node.distance,
            })
            .collect();
        Ok(Self(stations))
    }
}
