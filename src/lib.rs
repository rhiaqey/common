use crate::error::RhiaqeyError;

pub mod client;
pub mod env;
pub mod error;
pub mod executor;
pub mod pubsub;
pub mod redis;
pub mod redis_rs;
pub mod security;
pub mod stream;
pub mod topics;

pub type RhiaqeyResult<T> = Result<T, RhiaqeyError>;
