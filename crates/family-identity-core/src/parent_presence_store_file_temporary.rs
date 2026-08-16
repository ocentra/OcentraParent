use std::fs::{self, File, OpenOptions};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::parent_presence_store::ParentPresenceStoreError;
use crate::parent_presence_store_file_platform::{
    configure_private_creation, validate_private_store_metadata,
};

pub(crate) fn reserve_private_temporary_artifact(
    path: &Path,
) -> Result<TemporaryStoreArtifact, ParentPresenceStoreError> {
    let parent = path.parent().ok_or(ParentPresenceStoreError::Unavailable)?;
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or(ParentPresenceStoreError::Unavailable)?;
    for _attempt in 0..16 {
        let candidate = private_temporary_candidate(parent, file_name)?;
        match create_private_file(&candidate) {
            Ok(file) => return TemporaryStoreArtifact::new(candidate, file),
            Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
            Err(_error) => return Err(ParentPresenceStoreError::Unavailable),
        }
    }
    Err(ParentPresenceStoreError::Unavailable)
}

fn private_temporary_candidate(
    parent: &Path,
    file_name: &str,
) -> Result<PathBuf, ParentPresenceStoreError> {
    let mut random = [0_u8; 16];
    getrandom::fill(&mut random).map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    Ok(parent.join(format!(".{file_name}.initialize-{}", encode_hex(&random))))
}

fn create_private_file(path: &Path) -> std::io::Result<File> {
    let mut options = OpenOptions::new();
    options.read(true).write(true).create_new(true);
    configure_private_creation(&mut options);
    options.open(path)
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

pub(crate) struct TemporaryStoreArtifact {
    path: PathBuf,
    file: Option<File>,
    identity: Option<same_file::Handle>,
}

impl TemporaryStoreArtifact {
    fn new(path: PathBuf, file: File) -> Result<Self, ParentPresenceStoreError> {
        let identity = same_file::Handle::from_file(
            file.try_clone()
                .map_err(|_error| ParentPresenceStoreError::Unavailable)?,
        )
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        Ok(Self {
            path,
            file: Some(file),
            identity: Some(identity),
        })
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn validate_path_identity(&self) -> Result<(), ParentPresenceStoreError> {
        let metadata = fs::symlink_metadata(&self.path)
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        validate_private_store_metadata(&metadata)?;
        let current = same_file::Handle::from_path(&self.path)
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        match self.identity.as_ref() {
            Some(identity) if current == *identity => Ok(()),
            _ => Err(ParentPresenceStoreError::IntegrityRejected),
        }
    }

    pub(crate) fn sync_all(&self) -> Result<(), ParentPresenceStoreError> {
        self.file
            .as_ref()
            .ok_or(ParentPresenceStoreError::Unavailable)?
            .sync_all()
            .map_err(|_error| ParentPresenceStoreError::Unavailable)
    }
}

impl Drop for TemporaryStoreArtifact {
    fn drop(&mut self) {
        drop(self.identity.take());
        drop(self.file.take());
        let _main = fs::remove_file(&self.path);
        for suffix in ["-journal", "-wal", "-shm"] {
            let mut sidecar = self.path.as_os_str().to_owned();
            sidecar.push(suffix);
            let _sidecar = fs::remove_file(PathBuf::from(sidecar));
        }
    }
}
