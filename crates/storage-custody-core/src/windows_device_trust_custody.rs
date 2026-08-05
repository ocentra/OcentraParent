//! Windows DPAPI custody with app-data records subordinate to a registry epoch.
use fs2::FileExt;
use getrandom::fill;
use ocentra_family_identity_core::trust_bootstrap::AwaitingPlatformKeySealingRequest;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

#[path = "windows_device_trust_custody_platform.rs"]
mod platform;
#[path = "windows_device_trust_custody_record.rs"]
mod record;
use record::{
    binding as custody_binding, hex, install_generation as load_or_create_install_generation,
    remove, write, Record,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Invalid,
    Platform,
    Missing,
    Mismatch,
    Io,
    Unseal,
}
pub struct WindowsDeviceTrustCustody {
    root: PathBuf,
    install_generation: String,
    binding_locks: Mutex<HashMap<String, Arc<Mutex<()>>>>,
}

impl WindowsDeviceTrustCustody {
    pub fn open(root: impl AsRef<Path>) -> Result<Self, Error> {
        if !cfg!(windows) {
            return Err(Error::Platform);
        }
        let root_was_absent = !root.as_ref().exists();
        fs::create_dir_all(root.as_ref()).map_err(|_error| Error::Io)?;
        if fs::symlink_metadata(root.as_ref())
            .map_err(|_error| Error::Io)?
            .file_type()
            .is_symlink()
        {
            return Err(Error::Invalid);
        }
        let root = root.as_ref().canonicalize().map_err(|_error| Error::Io)?;
        Ok(Self {
            install_generation: load_or_create_install_generation(&root, root_was_absent)?,
            root,
            binding_locks: Mutex::new(HashMap::new()),
        })
    }
    pub fn seal_persist_activate(
        &self,
        request: AwaitingPlatformKeySealingRequest,
        material: &[u8],
    ) -> Result<(), Error> {
        if material.is_empty() {
            return Err(Error::Invalid);
        }
        let binding = custody_binding([
            &request.family_id,
            &request.parent_account_id,
            &request.device_ref,
            &self.install_generation,
        ])?;
        let binding_lock = self.binding_lock(&binding);
        let _binding_guard = binding_lock
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _binding_fence = self.binding_fence(&binding)?;
        let mut epoch = [0; 32];
        fill(&mut epoch).map_err(|_error| Error::Platform)?;
        let record = Record {
            family: request.family_id,
            account: request.parent_account_id,
            device: request.device_ref,
            epoch_hash: hex(Sha256::digest(epoch)),
            ciphertext: platform::protect(material, &binding)?,
        };
        let record_path = self.path(&binding);
        write(&record_path, &record)?;
        if let Err(error) = platform::activate(&binding, &epoch) {
            let _cleanup_result = fs::remove_file(&record_path);
            return Err(error);
        }
        verify_activated_binding(&binding, &epoch, &record_path, platform::current(&binding))
    }
    pub fn unseal_current(&self, family: &str, account: &str, device: &str) -> Result<(), Error> {
        let b = custody_binding([family, account, device, &self.install_generation])?;
        let binding_lock = self.binding_lock(&b);
        let _binding_guard = binding_lock
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _binding_fence = self.binding_fence(&b)?;
        let r: Record =
            serde_json::from_slice(&fs::read(self.path(&b)).map_err(|_error| Error::Missing)?)
                .map_err(|_error| Error::Io)?;
        if r.family != family
            || r.account != account
            || r.device != device
            || r.epoch_hash != hex(Sha256::digest(platform::current(&b)?))
        {
            return Err(Error::Mismatch);
        }
        platform::unprotect(&r.ciphertext, &b).map(|_plaintext| ())
    }
    pub fn revoke_or_reset(&self, family: &str, account: &str, device: &str) -> Result<(), Error> {
        let b = custody_binding([family, account, device, &self.install_generation])?;
        let binding_lock = self.binding_lock(&b);
        let _binding_guard = binding_lock
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _binding_fence = self.binding_fence(&b)?;
        match platform::remove(&b) {
            Ok(()) | Err(Error::Missing) => {}
            Err(error) => return Err(error),
        }
        remove(&self.path(&b))
    }
    fn path(&self, b: &[u8]) -> PathBuf {
        self.root.join(format!("{}.sealed", hex(Sha256::digest(b))))
    }

    fn binding_fence(&self, binding: &[u8]) -> Result<fs::File, Error> {
        let fence = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(
                self.root
                    .join(format!("{}.lock", hex(Sha256::digest(binding)))),
            )
            .map_err(|_error| Error::Io)?;
        fence.lock_exclusive().map_err(|_error| Error::Io)?;
        Ok(fence)
    }

    fn binding_lock(&self, binding: &[u8]) -> Arc<Mutex<()>> {
        let key = hex(Sha256::digest(binding));
        let mut locks = self
            .binding_locks
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        Arc::clone(locks.entry(key).or_insert_with(|| Arc::new(Mutex::new(()))))
    }
}

fn verify_activated_binding(
    binding: &[u8],
    epoch: &[u8],
    record_path: &Path,
    verification: Result<Vec<u8>, Error>,
) -> Result<(), Error> {
    match verification {
        Ok(current) if current == epoch => Ok(()),
        Ok(_) => {
            rollback_activated_binding(binding, record_path);
            Err(Error::Mismatch)
        }
        Err(error) => {
            rollback_activated_binding(binding, record_path);
            Err(error)
        }
    }
}

fn rollback_activated_binding(binding: &[u8], record_path: &Path) {
    let _cleanup_result = platform::remove(binding);
    let _cleanup_result = fs::remove_file(record_path);
}

#[cfg(all(test, windows))]
mod tests {
    use super::{verify_activated_binding, Error};
    use std::fs;

    #[test]
    fn verification_read_error_removes_the_persisted_record() -> Result<(), String> {
        let root = std::env::temp_dir().join(format!(
            "ocentra-wp02-post-activation-cleanup-{}",
            std::process::id()
        ));
        let _cleanup = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).map_err(|error| format!("create root: {error}"))?;
        let record_path = root.join("record.sealed");
        fs::write(&record_path, "sealed").map_err(|error| format!("write record: {error}"))?;

        assert_eq!(
            verify_activated_binding(
                b"post-activation-cleanup",
                b"epoch",
                &record_path,
                Err(Error::Platform)
            ),
            Err(Error::Platform)
        );
        assert!(
            !record_path.exists(),
            "a verification read failure must remove the persisted record"
        );

        let _cleanup = fs::remove_dir_all(root);
        Ok(())
    }
}
