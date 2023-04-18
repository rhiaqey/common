use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RhiaqeyError {
    pub code: Option<i32>,
    pub message: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub error: Option<Box<dyn Error>>,
}

impl Debug for RhiaqeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Display for RhiaqeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for RhiaqeyError {
    //
}

impl RhiaqeyError {
    pub fn create(code: i32, message: String) -> RhiaqeyError {
        RhiaqeyError { code: Some(code), message, error: None }
    }
}

impl From<String> for RhiaqeyError {
    fn from(message: String) -> Self {
        RhiaqeyError { code: None, message, error: None }
    }
}

impl From<(i32, String)> for RhiaqeyError {
    fn from(message: (i32, String)) -> Self {
        RhiaqeyError { code: Some(message.0), message: message.1, error: None }
    }
}

impl From<std::io::Error> for RhiaqeyError {
    fn from(value: std::io::Error) -> Self {
        RhiaqeyError{
            code: None,
            message: value.to_string(),
            error: Some(Box::new(value))
        }
    }
}

impl From<rustis::Error> for RhiaqeyError {
    fn from(value: rustis::Error) -> Self {
        RhiaqeyError{
            code: None,
            message: value.to_string(),
            error: Some(Box::new(value))
        }
    }
}

impl From<serde_json::Error> for RhiaqeyError {
    fn from(value: serde_json::Error) -> Self {
        RhiaqeyError{
            code: None,
            message: value.to_string(),
            error: Some(Box::new(value))
        }
    }
}

#[cfg(feature = "rss")]
impl From<rss::Error> for RhiaqeyError {
    fn from(value: rss::Error) -> Self {
        RhiaqeyError{
            code: None,
            message: value.to_string(),
            error: Some(Box::new(value))
        }
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for RhiaqeyError {
    fn from(value: reqwest::Error) -> Self {
        RhiaqeyError{
            code: None,
            message: value.to_string(),
            error: Some(Box::new(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::{RhiaqeyError};

    #[test]
    fn can_serialize() {
        let err = RhiaqeyError::create(4545, "We have some error message".to_string());
        let result = serde_json::to_string(&err);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "{\"code\":4545,\"message\":\"We have some error message\"}")
    }
}
