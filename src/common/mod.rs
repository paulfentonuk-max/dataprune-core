//! Common utilities and types

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("crypto error: {0}")]
    Crypto(String),
    
    #[error("protocol error: {0}")]
    Protocol(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
