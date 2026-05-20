#[derive(Debug)]
pub enum JournalError {
    Crypto,
    Decode(base64::DecodeError),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for JournalError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for JournalError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<base64::DecodeError> for JournalError {
    fn from(error: base64::DecodeError) -> Self {
        Self::Decode(error)
    }
}
