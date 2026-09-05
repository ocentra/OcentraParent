#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActivityDbPath(pub std::path::PathBuf);

impl AsRef<std::path::Path> for ActivityDbPath {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_path()
    }
}

impl From<ActivityDbPath> for std::path::PathBuf {
    fn from(value: ActivityDbPath) -> Self {
        value.0
    }
}
