use crate::err::Result;
use crate::tile::Tile;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{SqlitePool, migrate};
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::TcpListener;

pub fn get_var(var_name: &str) -> Result<String> {
    env::var(var_name).map_err(|_| format!("environment variable '{}' missing", var_name).into())
}

/// Config variables related to digitransit
#[derive(Debug)]
pub struct DigitransitConf {
    routing_url: String,
    img_url: String,
    api_key: String,
}

impl DigitransitConf {
    fn from_env() -> Result<Self> {
        Ok(Self {
            routing_url: get_var("DIGITRANSIT_ROUTING_URL")?,
            img_url: get_var("DIGITRANSIT_IMG_URL")?,
            api_key: get_var("DIGITRANSIT_API_KEY")?,
        })
    }

    /// Request for nearby bike stations
    pub async fn nearby_request(
        &self,
        lon: f64,
        lat: f64,
        max_distance: u16,
        max_results: u8,
    ) -> Result<reqwest::Response> {
        Ok(reqwest::Client::new()
            .post(&self.routing_url)
            .header(reqwest::header::CONTENT_TYPE, "application/graphql")
            .header("digitransit-subscription-key", &self.api_key)
            .body(nearest_query(lon, lat, max_distance, max_results))
            .send()
            .await?)
    }

    /// Request for tile image (as png)
    pub async fn img_request(&self, tile: &Tile) -> Result<reqwest::Response> {
        let url = tile.digitransit_url(&self.img_url);
        Ok(reqwest::Client::new()
            .get(url)
            .header("digitransit-subscription-key", &self.api_key)
            .send()
            .await?)
    }
}

fn nearest_query(lon: f64, lat: f64, max_distance: u16, max_results: u8) -> String {
    format!(
        r#"
{{
  nearest(
    lon: {}, lat: {}, maxDistance: {}, maxResults: {},
    filterByPlaceTypes: [VEHICLE_RENT],
    filterByModes: [BICYCLE]
    filterByNetwork: ["smoove", "vantaa"]
  ) {{
    edges {{
      node {{
        distance
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
      }}
    }}
  }}
}}
    "#,
        lon, lat, max_distance, max_results
    )
}

/// Config variables related to the app itself
#[derive(Debug)]
pub struct AppConf {
    db_url: String,
    port: u16,
}

impl AppConf {
    fn from_env() -> Result<Self> {
        Ok(Self {
            db_url: get_var("DATABASE_URL")?,
            port: get_var("PORT")?.parse()?,
        })
    }

    pub async fn con_pool(&self) -> Result<SqlitePool> {
        let opt = SqliteConnectOptions::from_str(&self.db_url)?.create_if_missing(true);
        let pool = SqlitePool::connect_with(opt).await?;

        migrate!().run(&pool).await?;
        Ok(pool)
    }

    pub async fn listener(&self) -> Result<TcpListener> {
        Ok(TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], self.port))).await?)
    }
}

/// Read app and digitransit configuration variables from the environment
pub fn get_conf() -> Result<(AppConf, DigitransitConf)> {
    Ok((AppConf::from_env()?, DigitransitConf::from_env()?))
}
