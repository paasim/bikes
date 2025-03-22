use std::{error, fmt, io, num, string, time};

#[derive(Debug)]
pub enum Error {
    Askama(askama::Error),
    Axum(axum::Error),
    AxumHttp(axum::http::Error),
    Reqwest(reqwest::Error),
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Matches on a Result, in case on Err, logs it, turns it a into response and returns it.
#[macro_export]
macro_rules! err_to_resp {
    ($e:expr) => {
        match $e {
            Ok(val) => val,
            Err(err) => {
                let err = $crate::err::Error::from(err);
                tracing::error!("{err}");
                return err.into_response();
            }
        }
    };
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Askama(e) => write!(f, "{}", e),
            Error::Axum(e) => write!(f, "{}", e),
            Error::AxumHttp(e) => write!(f, "{}", e),
            Error::Reqwest(e) => write!(f, "{}", e),
            Error::Other(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {}

impl From<askama::Error> for Error {
    fn from(value: askama::Error) -> Self {
        Self::Askama(value)
    }
}

impl From<axum::Error> for Error {
    fn from(value: axum::Error) -> Self {
        Self::Axum(value)
    }
}

impl From<axum::http::Error> for Error {
    fn from(value: axum::http::Error) -> Self {
        Self::AxumHttp(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(value: string::FromUtf8Error) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<core::num::ParseIntError> for Error {
    fn from(value: core::num::ParseIntError) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Other(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Other(value.to_owned())
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<time::SystemTimeError> for Error {
    fn from(value: time::SystemTimeError) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<num::TryFromIntError> for Error {
    fn from(value: num::TryFromIntError) -> Self {
        Self::Other(value.to_string())
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_string(),
        )
            .into_response()
    }
}
