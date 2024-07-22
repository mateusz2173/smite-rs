use serde_json::Value;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error during making request.")]
    Reqwest(#[from] reqwest::Error),
    #[error("Error during parsing a request.")]
    Parsing(#[from] serde_json::Error),
    #[error("Unknown response: {0}")]
    UnknownResponse(Value),
    #[error("Session is required.")]
    Session,
    #[error("Smite API error: {0}")]
    SmiteApi(String),
}
