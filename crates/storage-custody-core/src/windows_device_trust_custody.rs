//! Windows DPAPI custody with app-data records subordinate to a registry epoch.
use atomicwrites::{AllowOverwrite, AtomicFile};
use getrandom::fill;
use ocentra_family_identity_core::trust_bootstrap::AwaitingPlatformKeySealingRequest;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[path = "windows_device_trust_custody_platform.rs"]
mod platform;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Invalid,
    Platform,
    Missing,
    Mismatch,
    Io,
    Unseal,
}
#[derive(Serialize, Deserialize)]
struct Record {
    family: String,
    account: String,
    device: String,
    epoch_hash: String,
    ciphertext: Vec<u8>,
}
pub struct WindowsDeviceTrustCustody {
    root: PathBuf,
}

impl WindowsDeviceTrustCustody {
    pub fn open(root: impl AsRef<Path>) -> Result<Self, Error> {
        fs::create_dir_all(root.as_ref()).map_err(|_error| Error::Io)?;
        if fs::symlink_metadata(root.as_ref())
            .map_err(|_error| Error::Io)?
            .file_type()
            .is_symlink()
        {
            return Err(Error::Invalid);
        }
        Ok(Self {
            root: root.as_ref().canonicalize().map_err(|_error| Error::Io)?,
        })
    }
    pub fn seal_persist_activate(
        &self,
        request: AwaitingPlatformKeySealingRequest,
        material: &[u8],
    ) -> Result<(), Error> {
        if material.is_empty()
            || !valid(&request.family_id)
            || !valid(&request.parent_account_id)
            || !valid(&request.device_ref)
        {
            return Err(Error::Invalid);
        }
        let binding = binding(
            &request.family_id,
            &request.parent_account_id,
            &request.device_ref,
        );
        let mut epoch = [0; 32];
        fill(&mut epoch).map_err(|_error| Error::Platform)?;
        let record = Record {
            family: request.family_id,
            account: request.parent_account_id,
            device: request.device_ref,
            epoch_hash: hex(Sha256::digest(epoch)),
            ciphertext: platform::protect(material, &binding)?,
        };
        write(&self.path(&binding), &record)?;
        platform::activate(&binding, &epoch)
    }
    pub fn unseal_current(
        &self,
        family: &str,
        account: &str,
        device: &str,
    ) -> Result<Vec<u8>, Error> {
        let b = binding(family, account, device);
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
        platform::unprotect(&r.ciphertext, &b)
    }
    pub fn revoke_or_reset(&self, family: &str, account: &str, device: &str) -> Result<(), Error> {
        let b = binding(family, account, device);
        platform::remove(&b)?;
        let _ = fs::remove_file(self.path(&b));
        Ok(())
    }
    fn path(&self, b: &[u8]) -> PathBuf {
        self.root.join(format!("{}.sealed", hex(Sha256::digest(b))))
    }
}
fn valid(v: &str) -> bool {
    !v.trim().is_empty()
}
fn binding(f: &str, a: &str, d: &str) -> Vec<u8> {
    [f, a, d].join("\u{1f}").into_bytes()
}
fn hex(bytes: impl AsRef<[u8]>) -> String {
    bytes.as_ref().iter().map(|b| format!("{b:02x}")).collect()
}
fn write(path: &Path, r: &Record) -> Result<(), Error> {
    AtomicFile::new(path, AllowOverwrite)
        .write(|f| {
            serde_json::to_writer(&mut *f, r).map_err(io::Error::other)?;
            f.sync_all()
        })
        .map_err(|_error| Error::Io)
}
