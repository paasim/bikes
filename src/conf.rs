use crate::err::Result;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{SqlitePool, migrate};
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::TcpListener;

pub const DIGITRANSIT_ROUTING_URL: &str = "https://api.digitransit.fi/routing/v2/hsl/gtfs/v1";
pub const DIGITRANSIT_IMG_URL: &str = "https://cdn.digitransit.fi/map/v3/hsl-map";

/// Config variables related to the app itself
#[derive(Debug)]
pub struct AppConf {
    api_key: String,
    db_url: String,
    port: u16,
}

pub fn get_var(var_name: &str) -> Result<String> {
    env::var(var_name).map_err(|_| format!("environment variable '{}' missing", var_name).into())
}

impl AppConf {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            db_url: get_var("DATABASE_URL")?,
            port: get_var("PORT")?.parse()?,
            api_key: get_var("DIGITRANSIT_API_KEY")?,
        })
    }

    /// run last as this takes AppConf as owned
    pub fn api_key(self) -> String {
        self.api_key
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
