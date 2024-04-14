use crate::err::Res;
use crate::tile::Tile;
use std::env;
use std::future::Future;

pub fn get_var(var_name: &str) -> Res<String> {
    env::var(var_name).map_err(|_| format!("environment variable '{}' missing", var_name).into())
}

#[derive(Debug)]
pub struct DtConf {
    routing_url: String,
    img_url: String,
    api_key: String,
}

impl DtConf {
    fn from_env() -> Res<Self> {
        Ok(Self {
            routing_url: get_var("DIGITRANSIT_ROUTING_URL")?,
            img_url: get_var("DIGITRANSIT_IMG_URL")?,
            api_key: get_var("DIGITRANSIT_API_KEY")?,
        })
    }

    pub fn nearby_request(
        &self,
        lon: f64,
        lat: f64,
        max_distance: u16,
        max_results: u8,
    ) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> {
        reqwest::Client::new()
            .post(&self.routing_url)
            .header(reqwest::header::CONTENT_TYPE, "application/graphql")
            .header("digitransit-subscription-key", &self.api_key)
            .body(nearest_query(lon, lat, max_distance, max_results))
            .send()
    }

    pub fn img_request(
        &self,
        tile: &Tile,
    ) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> {
        let url = tile.digitransit_url(&self.img_url);
        reqwest::Client::new()
            .get(url)
            .header("digitransit-subscription-key", &self.api_key)
            .send()
    }
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

#[derive(Debug)]
pub struct Conf {
    pub db_url: String,
    pub port: u16,
}

impl Conf {
    fn from_env() -> Res<Self> {
        Ok(Self {
            db_url: get_var("DATABASE_URL")?,
            port: get_var("PORT")?.parse()?,
        })
    }
}

pub fn get_conf() -> Res<(Conf, DtConf)> {
    Ok((Conf::from_env()?, DtConf::from_env()?))
}
