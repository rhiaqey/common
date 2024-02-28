use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum RhiaqeyError {
    Other(String),
    IO(std::io::Error),
    Redis(rustis::Error),
    Serde(serde_json::Error),

    #[cfg(feature = "rss")]
    RSS(rss::Error),

    #[cfg(feature = "reqwest")]
    Reqwest(reqwest::Error),

    #[cfg(feature = "quick-xml")]
    QuickXML(quick_xml::Error),

    #[cfg(feature = "quick-xml")]
    QuickXMLDeserialization(quick_xml::DeError),
}

impl Display for RhiaqeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RhiaqeyError::Other(err) => write!(f, "{}", err),
            RhiaqeyError::IO(err) =>write!(f, "{}", err),
            RhiaqeyError::Redis(err) => write!(f, "{}", err),
            RhiaqeyError::Serde(err) => write!(f, "{}", err),
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

impl From<serde_json::Error> for RhiaqeyError {
    fn from(value: serde_json::Error) -> Self {
        RhiaqeyError::Serde(value)
    }
}

#[cfg(feature = "rss")]
impl From<rss::Error> for RhiaqeyError {
    fn from(value: rss::Error) -> Self {
        RhiaqeyError::RSS(value)
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for RhiaqeyError {
    fn from(value: reqwest::Error) -> Self {
        RhiaqeyError::Reqwest(value)
    }
}

#[cfg(feature = "quick-xml")]
impl From<quick_xml::Error> for RhiaqeyError {
    fn from(value: quick_xml::Error) -> Self {
        RhiaqeyError::QuickXML(value)
    }
}

#[cfg(feature = "quick-xml")]
impl From<quick_xml::DeError> for RhiaqeyError {
    fn from(value: quick_xml::DeError) -> Self {
        RhiaqeyError::QuickXMLDeserialization(value)
    }
}
