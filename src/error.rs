use std::fmt::Display;
use std::fmt::Formatter;
use std::string::ToString;

#[derive(Debug)]
pub enum OnfidoError {
    Reqwest(reqwest::Error),
    Parse(std::string::ParseError),
    Header(reqwest::header::InvalidHeaderValue),
    Io(std::io::Error),
    Env(std::env::VarError),
    Serialiaze(serde_json::Error),
    Url(reqwest::UrlError),
    FileType,
}

impl Display for OnfidoError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let message = match self {
            OnfidoError::Reqwest(e) => e.to_string(),
            OnfidoError::Parse(e) => e.to_string(),
            OnfidoError::Header(e) => e.to_string(),
            OnfidoError::Io(e) => e.to_string(),
            OnfidoError::Serialiaze(e) => e.to_string(),
            OnfidoError::Env(e) => e.to_string(),
            OnfidoError::Url(e) => e.to_string(),
            OnfidoError::FileType => "Wrong file type".to_string(),
        };
        write!(f, "{}", message)
    }
}

impl From<reqwest::Error> for OnfidoError {
    fn from(e: reqwest::Error) -> Self {
        OnfidoError::Reqwest(e)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for OnfidoError {
    fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
        OnfidoError::Header(e)
    }
}

impl From<std::io::Error> for OnfidoError {
    fn from(e: std::io::Error) -> Self {
        OnfidoError::Io(e)
    }
}

impl From<std::env::VarError> for OnfidoError {
    fn from(e: std::env::VarError) -> Self {
        OnfidoError::Env(e)
    }
}

impl From<serde_json::Error> for OnfidoError {
    fn from(e: serde_json::Error) -> Self {
        OnfidoError::Serialiaze(e)
    }
}

impl From<reqwest::UrlError> for OnfidoError {
    fn from(e: reqwest::UrlError) -> Self {
        OnfidoError::Url(e)
    }
}
