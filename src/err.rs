use std::{error, fmt, io, num, string, time};

#[derive(Debug)]
pub enum AppError {
    Askama(askama::Error),
    Axum(axum::Error),
    AxumHttp(axum::http::Error),
    Reqwest(reqwest::Error),
    Other(String),
}

pub type Res<T> = Result<T, AppError>;

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Askama(e) => write!(f, "{}", e),
            AppError::Axum(e) => write!(f, "{}", e),
            AppError::AxumHttp(e) => write!(f, "{}", e),
            AppError::Reqwest(e) => write!(f, "{}", e),
            AppError::Other(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for AppError {}

impl From<askama::Error> for AppError {
    fn from(value: askama::Error) -> Self {
        Self::Askama(value)
    }
}

impl From<axum::Error> for AppError {
    fn from(value: axum::Error) -> Self {
        Self::Axum(value)
    }
}

impl From<axum::http::Error> for AppError {
    fn from(value: axum::http::Error) -> Self {
        Self::AxumHttp(value)
    }
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<string::FromUtf8Error> for AppError {
    fn from(value: string::FromUtf8Error) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<core::num::ParseIntError> for AppError {
    fn from(value: core::num::ParseIntError) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<String> for AppError {
    fn from(value: String) -> Self {
        Self::Other(value)
    }
}

impl From<&str> for AppError {
    fn from(value: &str) -> Self {
        Self::Other(value.to_owned())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<sqlx::migrate::MigrateError> for AppError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<time::SystemTimeError> for AppError {
    fn from(value: time::SystemTimeError) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<num::TryFromIntError> for AppError {
    fn from(value: num::TryFromIntError) -> Self {
        Self::Other(value.to_string())
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{}", self);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_string(),
        )
            .into_response()
    }
}
