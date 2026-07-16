use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpdaterError {
    #[error("{0}")]
    Usage(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Semver(#[from] semver::Error),
    #[error("{0}")]
    Crypto(String),
    #[error("{0}")]
    Policy(String),
    #[error("{0}")]
    Process(String),
}
