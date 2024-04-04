use axum_core::response::{IntoResponse, Response};
use hyper::StatusCode;
use std::fmt::{Debug, Display};
use std::str::Utf8Error;
use std::string::FromUtf8Error;

use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_json::json;

#[derive(Debug)]
pub enum RhiaqeyError {
    Other(String),
    IO(std::io::Error),
    Redis(rustis::Error),
    RedisRs(redis::RedisError),
    Serde(serde_json::Error),
    UTF8Error(Utf8Error),

    #[cfg(feature = "rss")]
    RSS(rss::Error),

    #[cfg(feature = "reqwest")]
    Reqwest(reqwest::Error),

    #[cfg(feature = "quick-xml")]
    QuickXML(quick_xml::Error),

    #[cfg(feature = "quick-xml")]
    QuickXMLDeserialization(quick_xml::DeError),
}

impl std::error::Error for RhiaqeyError {}

impl Display for RhiaqeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RhiaqeyError::Other(err) => write!(f, "{}", err),
            RhiaqeyError::IO(err) => write!(f, "{}", err),
            RhiaqeyError::Redis(err) => write!(f, "{}", err),
            RhiaqeyError::RedisRs(err) => write!(f, "{}", err),
            RhiaqeyError::Serde(err) => write!(f, "{}", err),
            RhiaqeyError::UTF8Error(err) => write!(f, "{}", err),
            #[cfg(feature = "rss")]
            RhiaqeyError::RSS(err) => write!(f, "{}", err),
            #[cfg(feature = "reqwest")]
            RhiaqeyError::Reqwest(err) => write!(f, "{}", err),
            #[cfg(feature = "quick-xml")]
            RhiaqeyError::QuickXML(err) => write!(f, "{}", err),
            #[cfg(feature = "quick-xml")]
            RhiaqeyError::QuickXMLDeserialization(err) => write!(f, "{}", err),
        }
    }
}

impl RhiaqeyError {
    pub fn kind(&self) -> &str {
        match self {
            RhiaqeyError::Other(_) => "other",
            RhiaqeyError::IO(_) => "io",
            RhiaqeyError::Redis(_) => "redis",
            RhiaqeyError::RedisRs(_) => "redis",
            RhiaqeyError::Serde(_) => "serde",
            RhiaqeyError::UTF8Error(_) => "utf8",
            #[cfg(feature = "rss")]
            RhiaqeyError::RSS(_) => "rss",
            #[cfg(feature = "reqwest")]
            RhiaqeyError::Reqwest(_) => "http-request",
            #[cfg(feature = "quick-xml")]
            RhiaqeyError::QuickXML(_) => "xml",
            #[cfg(feature = "quick-xml")]
            RhiaqeyError::QuickXMLDeserialization(_) => "xml-deserialization",
        }
    }
}

impl Serialize for RhiaqeyError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("RhiaqeyError", 2)?;
        state.serialize_field("kind", &self.kind())?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

impl From<&str> for RhiaqeyError {
    fn from(message: &str) -> Self {
        RhiaqeyError::Other(message.into())
    }
}

impl From<String> for RhiaqeyError {
    fn from(message: String) -> Self {
        RhiaqeyError::Other(message)
    }
}

impl From<std::io::Error> for RhiaqeyError {
    fn from(value: std::io::Error) -> Self {
        RhiaqeyError::IO(value)
    }
}

impl From<rustis::Error> for RhiaqeyError {
    fn from(value: rustis::Error) -> Self {
        RhiaqeyError::Redis(value)
    }
}

impl From<redis::RedisError> for RhiaqeyError {
    fn from(value: redis::RedisError) -> Self {
        RhiaqeyError::RedisRs(value)
    }
}

impl From<serde_json::Error> for RhiaqeyError {
    fn from(value: serde_json::Error) -> Self {
        RhiaqeyError::Serde(value)
    }
}

impl From<Utf8Error> for RhiaqeyError {
    fn from(value: Utf8Error) -> Self {
        Self::UTF8Error(value)
    }
}

impl From<FromUtf8Error> for RhiaqeyError {
    fn from(value: FromUtf8Error) -> Self {
        Self::UTF8Error(value.utf8_error())
    }
}

#[cfg(feature = "rss")]
impl From<rss::Error> for RhiaqeyError {
    fn from(value: rss::Error) -> Self {
        Self::RSS(value)
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for RhiaqeyError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

#[cfg(feature = "quick-xml")]
impl From<quick_xml::Error> for RhiaqeyError {
    fn from(value: quick_xml::Error) -> Self {
        Self::QuickXML(value)
    }
}

#[cfg(feature = "quick-xml")]
impl From<quick_xml::DeError> for RhiaqeyError {
    fn from(value: quick_xml::DeError) -> Self {
        Self::QuickXMLDeserialization(value)
    }
}

impl IntoResponse for RhiaqeyError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(hyper::header::CONTENT_TYPE, "application/json")],
            json!({
                "message": format!("{}", self),
                "kind": self.kind()
            })
            .to_string(),
        )
            .into_response()
    }
}
