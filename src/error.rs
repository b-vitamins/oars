use reqwest::Error as ReqwestError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OARSError {
    #[error("Network error occurred: {0}")]
    NetworkError(#[from] ReqwestError),
    #[error("Daily query limit reached")]
    QueryLimitReached,
    #[error("Unknown error occurred")]
    Unknown,
}
