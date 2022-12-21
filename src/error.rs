use thiserror::Error;
use warp::reject::Reject;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Dotenv Error")]
    DotEnvError(#[from] dotenv::Error),
    #[error("Reqwest Error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Std Error")]
    StdError(#[from] std::io::Error),
    #[error("HTTP Error")]
    Http(#[from] warp::http::Error),
}

impl Reject for Error {}
