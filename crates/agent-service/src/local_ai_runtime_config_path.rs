use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug)]
pub struct ConfiguredLocalPath {
    path: Option<PathBuf>,
    configured: bool,
    exists: bool,
    byte_size: Option<u64>,
}

impl ConfiguredLocalPath {
    pub(crate) fn from_path(path: Option<&Path>) -> Self {
        match path {
            Some(path) => Self::configured(path),
            None => Self {
                path: None,
                configured: false,
                exists: false,
                byte_size: None,
            },
        }
    }

    pub fn is_configured(&self) -> bool {
        self.configured
    }

    pub fn exists(&self) -> bool {
        self.exists
    }

    pub fn byte_size(&self) -> Option<u64> {
        self.byte_size
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    fn configured(path: &Path) -> Self {
        match fs::metadata(path) {
            Ok(metadata) => Self {
                path: Some(path.to_path_buf()),
                configured: true,
                exists: metadata.is_file(),
                byte_size: Some(metadata.len()),
            },
            Err(_) => Self {
                path: Some(path.to_path_buf()),
                configured: true,
                exists: false,
                byte_size: None,
            },
        }
    }
}
