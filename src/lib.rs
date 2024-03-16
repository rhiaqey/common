use crate::error::RhiaqeyError;

pub mod client;
pub mod env;
pub mod pubsub;
pub mod redis;
pub mod redis_rs;
pub mod stream;
pub mod topics;
pub mod executor;
pub mod error;
pub mod security;

pub type RhiaqeyResult<T> = Result<T, RhiaqeyError>;
