use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Dotenv Error")]
    DotEnvError(#[from] dotenv::Error),
    #[error("Reqwest Error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Std Error")]
    StdError(#[from] std::io::Error),
}
