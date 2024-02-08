use crate::err::Res;
use std::env;

pub fn get_var(var_name: &str) -> Res<String> {
    env::var(var_name).map_err(|_| format!("environment variable '{}' missing", var_name).into())
}

#[derive(Debug)]
pub struct DtConf {
    pub routing_url: String,
    pub img_url: String,
    pub api_key: String,
}

impl DtConf {
    fn from_env() -> Res<Self> {
        Ok(Self {
            routing_url: get_var("DIGITRANSIT_ROUTING_URL")?,
            img_url: get_var("DIGITRANSIT_IMG_URL")?,
            api_key: get_var("DIGITRANSIT_API_KEY")?,
        })
    }
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
