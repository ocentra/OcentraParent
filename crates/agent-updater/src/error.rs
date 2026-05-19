use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum UpdaterError {
    Usage(String),
    Io(std::io::Error),
    Json(serde_json::Error),
    Http(reqwest::Error),
    Semver(semver::Error),
    Crypto(String),
    Policy(String),
    Process(String),
}

impl Display for UpdaterError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Usage(message) => write!(formatter, "{message}"),
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Json(error) => write!(formatter, "{error}"),
            Self::Http(error) => write!(formatter, "{error}"),
            Self::Semver(error) => write!(formatter, "{error}"),
            Self::Crypto(message) => write!(formatter, "{message}"),
            Self::Policy(message) => write!(formatter, "{message}"),
            Self::Process(message) => write!(formatter, "{message}"),
        }
    }
}

impl std::error::Error for UpdaterError {}

impl From<std::io::Error> for UpdaterError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for UpdaterError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<reqwest::Error> for UpdaterError {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error)
    }
}

impl From<semver::Error> for UpdaterError {
    fn from(error: semver::Error) -> Self {
        Self::Semver(error)
    }
}
