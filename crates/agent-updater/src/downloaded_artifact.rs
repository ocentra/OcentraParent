use std::{env, fs, path::PathBuf};

use rand_core::{OsRng, RngCore};

use crate::error::UpdaterError;

pub(super) struct DownloadedArtifact {
    pub(super) root: PathBuf,
    pub(super) path: PathBuf,
}

impl DownloadedArtifact {
    pub(super) fn new(name: &str) -> Result<Self, UpdaterError> {
        let safe_name = safe_artifact_name(name)?;
        let root = allocate_root()?;
        Ok(Self {
            path: root.join(safe_name),
            root,
        })
    }
}

fn safe_artifact_name(name: &str) -> Result<&str, UpdaterError> {
    name.rsplit(['/', '\\'])
        .next()
        .filter(|candidate| *candidate == name)
        .ok_or_else(|| UpdaterError::Policy("artifact name is not a safe file name".to_owned()))
}

fn allocate_root() -> Result<PathBuf, UpdaterError> {
    let mut random = [0_u8; 16];
    for _ in 0..8 {
        OsRng.fill_bytes(&mut random);
        let suffix = random
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let root = env::temp_dir().join(format!("ocentra-child-agent-update-{suffix}"));
        match fs::create_dir(&root) {
            Ok(()) => {
                return Ok(root);
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error.into()),
        }
    }
    Err(UpdaterError::Policy(
        "could not allocate a unique updater-owned temporary directory".to_owned(),
    ))
}

impl Drop for DownloadedArtifact {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
